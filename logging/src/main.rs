use death_god_logging_tool::logging_config::logging_config;

fn main() {
    std::env::set_var("LOG_LEVEL", "trace");

    logging_config("LOG_LEVEL");

    log::debug!("LOG");
    log::info!("INFO");
    log::warn!("WARN");
    log::error!("ERROR");
}
