use anyhow::Error;
use colored::*;
use flexi_logger::{Age, Duplicate, Logger};
#[cfg(unix)]
use libc;
#[cfg(unix)]
use std::os::unix::io::AsRawFd;

pub fn init(log_level: String, log_path: Option<String>) -> Result<(), Error> {
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
                    "  --> {}:{}",
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

            // Перенаправляем stdout и stderr в файл
            unsafe {
                if libc::dup2(console_file.as_raw_fd(), libc::STDOUT_FILENO) == -1 {
                    return Err(std::io::Error::last_os_error().into());
                }
                if libc::dup2(console_file.as_raw_fd(), libc::STDERR_FILENO) == -1 {
                    return Err(std::io::Error::last_os_error().into());
                }
            }
            // Закрываем оригинальные дескрипторы файлов
            drop(console_file);
        }
        #[cfg(windows)]
        {
            logger
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
                })
                .start()?;
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
