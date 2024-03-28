use death_god_logging_tool::logging_config::logging_config;

pub const VAR_NAME: &str = "LOG_LEVEL";

fn main() {
    std::env::set_var(VAR_NAME, "trace");

    logging_config(VAR_NAME);

    log::debug!("LOG");
    log::info!("INFO");
    log::warn!("WARN");
    log::error!("ERROR");
    log::trace!("long INFO ===================== ==================== =========================----------------------====================");
}
