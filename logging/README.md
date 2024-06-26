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
use death_god_logging_tool::logging_config::logging_config;

fn main() {
    std::env::set_var("LOG_LEVEL", "info,app=trace");
    // std::env::set_var("LOG_FILE_PATH", "/data/logs.txt");

    logging_config();

    log::debug!("LOG");
    log::info!("INFO");
    log::warn!("WARN");
    log::error!("ERROR");
}

```
```console
   Compiling death_god_logging_tool v1.0.3 (/home/death/my_projects/death_god_logging_tool/logging)
    Finished dev [unoptimized + debuginfo] target(s) in 0.55s
     Running `target/debug/death_god_logging_tool`
INFO   LOG_LEVEL=trace    
  --> src/logging_config.rs:54    2024-03-28T02:43:51
DEBUG  LOG                               
  --> src/main.rs:8    2024-03-28T02:43:51
INFO   INFO                              
  --> src/main.rs:9    2024-03-28T02:43:51
WARN   WARN                              
  --> src/main.rs:10    2024-03-28T02:43:51
ERROR  ERROR                             
  --> src/main.rs:11    2024-03-28T02:43:51
[Finished running. Exit status: 0]

```


![img.png](img.png)