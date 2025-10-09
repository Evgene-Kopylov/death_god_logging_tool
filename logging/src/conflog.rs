use colored::*;
use flexi_logger::{Age, Duplicate, Logger};
use anyhow::Error;
use std::fs::{create_dir_all, OpenOptions};
#[cfg(unix)]
use std::os::unix::io::AsRawFd;
#[cfg(unix)]
use libc;

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

            // выравнивание
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
                    .dimmed(),
            )
        });

    if let Some(path) = log_path {
        logger
            .log_to_file(flexi_logger::FileSpec::default().directory(path.clone()))
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

        // Дополнительно перенаправляем весь консольный вывод (stdout и stderr) в файлы.
        // Это позволит записывать любой вывод println!/eprintln!, паники и т.д.
        #[cfg(unix)]
        {
            let ts = chrono::Local::now().format("%Y-%m-%dT%H-%M-%S").to_string();

            // гарантируем, что директория логов существует
            create_dir_all(&path)?;

            let console_file_path = format!("{}/console.{}.log", &path, ts);

            let console_file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&console_file_path)?;

            unsafe {
                if libc::dup2(console_file.as_raw_fd(), libc::STDOUT_FILENO) == -1 {
                    return Err(std::io::Error::last_os_error().into());
                }
                if libc::dup2(console_file.as_raw_fd(), libc::STDERR_FILENO) == -1 {
                    return Err(std::io::Error::last_os_error().into());
                }
            }
            // Закрываем оригинальные дескрипторы файлов (переназначенные stdout/stderr уже активны).
            drop(console_file);
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
