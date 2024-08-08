use colored::*;
use env_logger;
use std::fs::OpenOptions;
use std::io::Write;

pub fn init() {
    let var_name = "LOG_LEVEL";
    let mut builder = env_logger::Builder::new();
    builder
        .format(|buf, record| {
            let level_str = match record.level() {
                log::Level::Trace => "TRACE".purple(),
                log::Level::Debug => "DEBUG".blue(),
                log::Level::Info => "INFO".green(),
                log::Level::Warn => "WARN".yellow(),
                log::Level::Error => "ERROR".red(),
            };

            let level_str = format!("{:<width$}", level_str, width = 5).dimmed();

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
        .parse_env(var_name);

    if let Ok(value) = std::env::var("LOG_FILE_PATH") {
        let log_file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(value)
            .expect("Failed to open log file");

        builder
            .target(env_logger::Target::Pipe(Box::new(log_file)))
            .init();
    } else {
        builder.init();
    }

    log::info!(
        "{}={}",
        var_name.blue(),
        std::env::var(var_name)
            .unwrap_or("<Перемпенная не определена.>".to_string())
            .green(),
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
