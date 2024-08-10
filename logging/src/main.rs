use death_god_logging_tool::conflog;

fn main() {
    // Print logs
    std::env::set_var("LOG_LEVEL", "error,death_god_logging_tool=debug");

    // conflog::init();

    // log::debug!("LOG - принт");
    // log::info!("INFO - принт");
    // log::warn!("WARN - принт");
    // log::error!("ERROR - принт");

    // Save logs to file. No print
    std::env::set_var("LOG_FILE_PATH", "./logs.txt");

    conflog::init();

    log::debug!("LOG - записть в лог-файл");
    log::info!("INFO - записть в лог-файл");
    log::warn!("WARN - записть в лог-файл");
    log::error!("ERROR - записть в лог-файл");

}
