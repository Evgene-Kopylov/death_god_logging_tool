use death_god_logging_tool::conflog;

fn main() {
    println!("1 - вывод до инициализации логгера");
    // conflog::init("trace".to_string(), Some("logs".to_string()), true).ok();
    conflog::init("trace".to_string(),None, true).ok();
    println!("2 - вывод после инициализации логгера");

    log::trace!("ttt - запись в лог-файл");
    println!("3 - обычный вывод между логами");

    log::debug!("ddd - запись в лог-файл");
    log::info!("i - запись в лог-файл");
    log::warn!("w - запись в лог-файл");
    log::error!("eee - запись в лог-файл");
    println!("print line ...");
    eprintln!("Это вывод в stderr");
}
