# death_god_logging_tool
 Настройка логирования с навигацией по коду.

## Крейт с настройками логов.

### Идея
Облегчить чтение большого количества коротких логов 
с одновременной навигацией по коду. Ссылки кликабельны в VsCode и Intellij.

### Настройка
Крейт берет преременные из окружения.

Уровень удобно указать в виде
```bash
LOG_LEVEL=warn,<your_app>=trace
```


Если указан абсолютный путь к лог файлу, запись будет производиться в него.
```bash
LOG_FILE_PATH=/data/logs.txt
```

### Пример

```toml
# Cargo.toml

[dependencies]
death_god_logging_tool = "1.x.x"
log = "x.x.x"
```

```rust
use death_god_logging_tool::conflog;

fn main() {
    // Print logs
    std::env::set_var("LOG_LEVEL", "error,death_god_logging_tool=debug");

    // Save logs to file. No print
    std::env::set_var("LOG_FILE_PATH", "./logs.txt");

    conflog::init();

    log::debug!("LOG - записть в лог-файл");
    log::info!("INFO - записть в лог-файл");
    log::warn!("WARN - записть в лог-файл");
    log::error!("ERROR - записть в лог-файл");

}

```
```console
DEBUG  LOG - принт                       
  --> src/main.rs:9    2024-08-10T08:16:54

INFO   INFO - принт                      
  --> src/main.rs:10    2024-08-10T08:16:54

WARN   WARN - принт                      
  --> src/main.rs:11    2024-08-10T08:16:54

ERROR  ERROR - принт                     
  --> src/main.rs:12    2024-08-10T08:16:54

```


![img.png](img.png)