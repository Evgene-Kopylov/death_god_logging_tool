use death_god_logging_tool::conflog;

fn main() {
    // Print logs
    std::env::set_var("LOG_LEVEL", "error,death_god_logging_tool=debug");

    std::env::set_var("LOG_PATH", "logs");
    conflog::init();

    log::trace!("ttt - записть в лог-файл");
    log::debug!("ddd - записть в лог-файл");
    log::info!("i - записть в лог-файл");
    log::warn!("w - записть в лог-файл");
    log::error!("eee - записть в лог-файл");
}
