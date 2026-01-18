use anyhow::Error;
use colored::*;
#[cfg(not(unix))]
use flexi_logger::Age;
use flexi_logger::{Duplicate, Logger};
#[cfg(unix)]
use libc;
#[cfg(unix)]
use std::os::unix::io::AsRawFd;

pub fn init(
    log_level: String,
    log_path: Option<String>,
    duplicate_to_console: bool,
) -> Result<(), Error> {
    let logger = Logger::try_with_str(log_level.clone())?
        .duplicate_to_stderr(Duplicate::All)
        .format_for_stderr(|buf, _now, record| {
            let level_str = match record.level() {
                log::Level::Trace => "TRACE".purple(),
                log::Level::Debug => "DEBUG".blue(),
                log::Level::Info => "INFO".green(),
                log::Level::Warn => "WARN".yellow(),
                log::Level::Error => "ERROR".red(),
            };

            // выравнивание для stderr
            let level_str = format!("{:<width$}", level_str, width = 5).dimmed();

            // собрать вместе
            writeln!(
                buf,
                "{}  {}    {}    {}",
                level_str,
                format_pprinted_string(record.args().to_string(), 30),
                format!(
                    "\n  --> {}:{}",
                    record.file().unwrap_or("unknown"),
                    record.line().unwrap_or(0)
                )
                .blue(),
                chrono::Local::now()
                    .format("%Y-%m-%dT%H:%M:%S")
                    .to_string()
                    .dimmed()
            )
        });

    if let Some(path) = log_path {
        #[cfg(unix)]
        {
            // Запускаем логгер только для stderr (без записи в файл)
            use std::{
                fs::{create_dir_all, OpenOptions},
                path::Path,
            };
            logger.start()?;

            // гарантируем, что директория логов существует
            create_dir_all(&path)?;

            // Получаем имя пакета из Cargo.toml
            let package_name = env!("CARGO_PKG_NAME");
            let current_file = format!("{}/{}_rCURRENT.log", &path, package_name);

            // Проверяем, существует ли CURRENT файл
            if Path::new(&current_file).exists() {
                // Находим все существующие файлы с номерами

                use std::fs::read_dir;
                let mut log_files: Vec<(i32, String)> = Vec::new();

                for entry in read_dir(&path)? {
                    let entry = entry?;
                    let file_name = entry.file_name().into_string().unwrap_or_default();

                    if file_name.starts_with(&format!("{}_r", package_name))
                        && file_name.ends_with(".log")
                        && file_name != format!("{}_rCURRENT.log", package_name)
                    {
                        // Извлекаем номер из имени файла
                        let num_part = file_name
                            .trim_start_matches(&format!("{}_r", package_name))
                            .trim_end_matches(".log");

                        if let Ok(num) = num_part.parse::<i32>() {
                            log_files.push((num, entry.path().to_string_lossy().to_string()));
                        }
                    }
                }

                // Сортируем по номерам
                log_files.sort_by_key(|(num, _)| *num);

                // Удаляем старые файлы, если их больше 4 (вместе с CURRENT будет 5)
                while log_files.len() >= 4 {
                    if let Some((_, oldest_file)) = log_files.pop() {
                        std::fs::remove_file(&oldest_file)?;
                    }
                }

                // Сдвигаем существующие файлы
                for (num, file_path) in log_files.iter_mut().rev() {
                    let new_num = *num + 1;
                    let new_name = format!("{}/{}_r{:05}.log", &path, package_name, new_num);
                    std::fs::rename(&file_path, &new_name)?;
                }

                // Переименовываем CURRENT в r00000
                let new_name = format!("{}/{}_r{:05}.log", &path, package_name, 0);
                std::fs::rename(&current_file, &new_name)?;
            }

            // Создаем новый CURRENT файл
            let console_file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&current_file)?;

            if duplicate_to_console {
                // Сохраняем оригинальные дескрипторы для дублирования вывода
                let original_stdout = unsafe { libc::dup(libc::STDOUT_FILENO) };
                let original_stderr = unsafe { libc::dup(libc::STDERR_FILENO) };

                // Создаем пайп для перехвата вывода
                let mut pipe_fds: [libc::c_int; 2] = [0; 2];
                unsafe {
                    if libc::pipe(pipe_fds.as_mut_ptr()) == -1 {
                        return Err(std::io::Error::last_os_error().into());
                    }
                }

                // Перенаправляем stdout и stderr в пайп
                unsafe {
                    if libc::dup2(pipe_fds[1], libc::STDOUT_FILENO) == -1 {
                        return Err(std::io::Error::last_os_error().into());
                    }
                    if libc::dup2(pipe_fds[1], libc::STDERR_FILENO) == -1 {
                        return Err(std::io::Error::last_os_error().into());
                    }
                    libc::close(pipe_fds[1]);
                }

                // Запускаем поток для чтения из пайпа и записи в файл и консоль
                // Нужно сохранить владение файлом, чтобы он не закрылся
                let current_file_clone = current_file.clone();
                std::thread::spawn(move || {
                    let mut buffer = [0u8; 4096];
                    loop {
                        unsafe {
                            let bytes_read = libc::read(
                                pipe_fds[0],
                                buffer.as_mut_ptr() as *mut _,
                                buffer.len(),
                            );
                            if bytes_read <= 0 {
                                break;
                            }

                            // Записываем в файл
                            let slice = &buffer[0..bytes_read as usize];
                            use std::io::Write;
                            let _ = std::fs::OpenOptions::new()
                                .create(true)
                                .append(true)
                                .open(&current_file_clone)
                                .and_then(|mut f| f.write_all(slice));

                            // Записываем в оригинальную консоль
                            let _ = libc::write(
                                original_stdout,
                                buffer.as_ptr() as *const _,
                                bytes_read as usize,
                            );
                        }
                    }
                    unsafe {
                        libc::close(pipe_fds[0]);
                        libc::close(original_stdout);
                        libc::close(original_stderr);
                    }
                });
            } else {
                // Просто перенаправляем stdout и stderr в файл
                unsafe {
                    if libc::dup2(console_file.as_raw_fd(), libc::STDOUT_FILENO) == -1 {
                        return Err(std::io::Error::last_os_error().into());
                    }
                    if libc::dup2(console_file.as_raw_fd(), libc::STDERR_FILENO) == -1 {
                        return Err(std::io::Error::last_os_error().into());
                    }
                }
            }
            // Закрываем оригинальные дескрипторы файлов
            drop(console_file);
        }
        #[cfg(not(unix))]
        {
            let mut logger_builder = logger
                .log_to_file(flexi_logger::FileSpec::default().directory(path))
                .rotate(
                    flexi_logger::Criterion::Age(Age::Day),
                    flexi_logger::Naming::Numbers,
                    flexi_logger::Cleanup::KeepLogFiles(7),
                )
                .format_for_files(|buf, _now, record| {
                    // выравнивание
                    let level_str = format!("{:<width$}", record.level(), width = 5);

                    let text_1 = format_pprinted_string(record.args().to_string(), 30);

                    let text_2 = format!(
                        "\n  --> {}:{}",
                        record.file().unwrap_or("unknown"),
                        record.line().unwrap_or(0)
                    );

                    // собрать вместе
                    writeln!(
                        buf,
                        "{}  {}    {}    {}",
                        level_str,
                        text_1,
                        text_2,
                        chrono::Local::now().format("%Y-%m-%dT%H:%M:%S")
                    )
                });

            // Для Windows: если нужно дублировать в консоль, оставляем duplicate_to_stderr
            // Если не нужно - отключаем дублирование
            if !duplicate_to_console {
                logger_builder = logger_builder.duplicate_to_stderr(Duplicate::None);
            }

            logger_builder.start()?;
        }
    } else {
        logger.start()?;
    }
    log::info!("LOG_LEVEL={}", log_level.clone());
    Ok(())
}

fn format_pprinted_string(original_string: String, desired_length: usize) -> String {
    let parts: Vec<&str> = original_string.split('\n').collect();

    if parts.len() >= 2 {
        let padded_second_part = format!(
            "{:<width$}",
            parts[parts.len() - 1],
            width = desired_length + 7
        );
        let result_string = format!(
            "{}\n{}",
            parts[..parts.len() - 1].join("\n"),
            padded_second_part
        );
        result_string
    } else {
        // В случае, если символ новой строки отсутствует, просто удлините всю строку
        let padded_string = format!("{:<width$}", original_string, width = desired_length);
        padded_string
    }
}
