use death_god_logging_tool::conflog;

fn main() {
    // Print logs
    std::env::set_var("LOG_LEVEL", "error,death_god_logging_tool=debug");

    std::env::set_var("LOG_PATH", "logs");
    conflog::init();

    log::trace!("TRACE - записть в лог-файл");
    log::debug!("LOG - записть в лог-файл");
    log::info!("INFO - записть в лог-файл");
    log::warn!("WARN - записть в лог-файл");
    log::error!("ERROR - записть в лог-файл");

}
