# death_god_logging_tool
 Настройка логирования с навигацией по коду.

## Крейт с настройками логов.

### Идея
Предназначен для облегчения чтения большого количества коротких логов.
В частности, при разработке логов.

Дает ссылку по коду.

### Настройка
Крейт берет преременную с уровнем логов из окружения.
Ее удобно указать в виде
```bash
LOG_LEVEL=info,<your_app>=trace
```
И использовать `log::trace!()` для отслеживания событий только в указанном проекте.

### Пример

```rust
use death_god_logging_tool::logging_config::logging_config;

fn main() {
    std::env::set_var("LOG_LEVEL", "trace");

    logging_config("LOG_LEVEL");

    log::debug!("LOG");
    log::info!("INFO");
    log::warn!("WARN");
    log::error!("ERROR");
}

```