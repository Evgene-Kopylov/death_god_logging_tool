use death_god_logging_tool::conflog;

fn main() {
    // Print logs
    let log_level = "error,death_god_logging_tool=debug";

    let log_path = "logs";
    // conflog::init(log_level.to_string(), Some(log_path.to_string()));
    conflog::init("trace".to_string(), None);

    log::trace!("ttt - записть в лог-файл");
    log::debug!("ddd - записть в лог-файл");
    log::info!("i - записть в лог-файл");
    log::warn!("w - записть в лог-файл");
    log::error!("eee - записть в лог-файл");
}
