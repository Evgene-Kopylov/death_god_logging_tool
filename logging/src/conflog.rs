use colored::*;
use env_logger;
// use std::fs::OpenOptions;
use std::io::Write;
use flexi_logger::{Age, Cleanup, Criterion, FileSpec, Logger, Naming};

pub fn init() {
    let mut builder = env_logger::Builder::new();
    builder
        // Формирование сообщения здесь
        // окраска
        .format(|buf, record| {
            let level_str = match record.level() {
                log::Level::Trace => "TRACE".purple(),
                log::Level::Debug => "DEBUG".blue(),
                log::Level::Info => "INFO".green(),
                log::Level::Warn => "WARN".yellow(),
                log::Level::Error => "ERROR".red(),
            };

            // выравнивание
            let level_str = format!("\n{:<width$}", level_str, width = 5).dimmed();

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
        })
        .parse_env("LOG_LEVEL");

    let log_level = std::env::var("LOG_LEVEL")
        .unwrap_or("info".to_string());

    if let Ok(path) = std::env::var("LOG_DIRECTORY_PATH") {
        println!("LOG_FILE_PATH={:?}", path);
        println!("LOG_LEVEL={:?}", log_level);
        Logger::try_with_str(log_level.clone())
            .unwrap()
            .log_to_file(
                flexi_logger::FileSpec::default().directory(path),
            )
            .rotate(
                flexi_logger::Criterion::Age(Age::Day),
                flexi_logger::Naming::Numbers,
                flexi_logger::Cleanup::KeepLogFiles(7),
            )
            .format_for_files(|buf, _now, record| {
                // собрать вместе
                writeln!(
                    buf,
                    "{}    {}    {}",
                    format_pprinted_string(record.args().to_string(), 30),
                    format!(
                        "\n  --> {}:{}",
                        record.file().unwrap_or("unknown"),
                        record.line().unwrap_or(0)
                    ),
                    chrono::Local::now()
                        .format("%Y-%m-%dT%H:%M:%S")
                        .to_string()
                )
            })
            .start()
            .unwrap();
    } else {
        builder.init();
    }

    log::info!(
        "LOG_LEVEL={}",
        log_level.clone()
    );
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