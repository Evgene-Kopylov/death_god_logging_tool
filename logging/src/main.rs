use death_god_logging_tool::conflog;

fn main() {
    // Используем новый параметр console_output для отображения в консоли
    // Теперь имя приложения определяется автоматически из std::env::current_exe()
    let console_output = cfg!(debug_assertions);
    println!("console_output: {}", console_output);
    conflog::init(
        "trace".to_string(),
        Some("logs".to_string()),
        console_output,
    )
    .ok();
    println!("2");

    log::trace!("ttt - записть в лог-файл");
    println!("3");

    log::debug!("ddd - записть в лог-файл");
    log::info!("i - записть в лог-файл");
    log::warn!("w - записть в лог-файл");
    log::error!("eee - записть в лог-файл");
    println!("print line ...");
    panic!("Паника!!!");
}
