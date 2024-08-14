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


Путь к директории для логов, по-умолчанию "logs".
Настроена ротация. По дням и 
```bash
LOG_PATH=logs/
```

### Пример

```toml
# Cargo.toml

[dependencies]
death_god_logging_tool = "1.x.x"
log = "x.x.x"
```

Example: [main.rs](src%2Fmain.rs)

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