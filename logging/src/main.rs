use death_god_logging_tool::logging_config::logging_config;

fn main() {
    std::env::set_var("LOG_LEVEL", "trace");
    // std::env::set_var("LOG_FILE_PATH", "./logs.txt");

    logging_config();

    log::debug!("LOG");
    log::info!("INFO");
    log::warn!("WARN");
    log::error!("ERROR");
}
