# Ao (アオ)

**Ao** — open-source локальный ИИ-ассистент, написанный на **Rust** и работающий с локальными языковыми моделями.

Проект создаётся как модульный ИИ-агент, способный работать с контекстом пользователя, файлами и внешними инструментами, сохраняя основную логику и данные локально.

Ao ориентирован на расширяемую архитектуру, независимость компонентов и возможность дальнейшего развития функциональности.

## Features

- 🧠 Local LLM
- 👤 Custom Profile & Personality
- ⚙️ Dynamic System Prompt
- 📁 File Context
- 🧩 Modular Architecture
- 💾 Conversation History & Memory — planned
- 🌐 Web Research — planned
- 🎙️ Voice Interaction — planned

## Architecture

```text
Input
  ↓
Command
  ↓
Request / Context
  ↓
SystemPrompt + Profile
  ↓
Tools
  ↓
Local LLM
  ↓
Response
```

### Components

| Component | Responsibility |
|---|---|
| `ao-core` | Основная логика ассистента |
| `ao-tui` | Терминальный пользовательский интерфейс |
| `ao-voice` | Голосовой ввод и вывод |
| `llama-server` | Локальный сервер языковой модели |

Архитектура находится в разработке и может изменяться.

## Tech Stack

- **Rust** — основной язык разработки
- **Tokio** — асинхронная среда выполнения
- **Reqwest** — HTTP-клиент
- **Serde** — сериализация и десериализация данных
- **Ratatui** — терминальный интерфейс
- **CPAL** — работа с аудиоустройствами
- **llama.cpp** — локальный инференс языковых моделей

## Installation

### Requirements

- Rust и Cargo
- llama.cpp / llama-server
- Локальная GGUF-модель
- Linux или Windows

### Build

Сборка конкретного пакета:

```bash
cargo build --release -p ao-tui
```

Общий синтаксис:

```bash
cargo build --release -p package-name
```

Исполняемый файл после сборки:

```text
target/release/ao-tui
```

## Running

### 1. Start llama-server

Сервер языковой модели можно запускать отдельно от Ao.

Пример:

```bash
llama-server \
  -m /path/to/model.gguf \
  --host 127.0.0.1 \
  --port 8080 \
  -c 4096 \
  -ngl 15
```

Проверка состояния:

```bash
curl http://127.0.0.1:8080/health
```

### 2. Start Ao

Запуск TUI:

```bash
./target/release/ao-tui
```

Если репозиторий находится в `~/repo/Ao`:

```bash
~/repo/Ao/target/release/ao-tui
```

### 3. Short command (Linux)

Для быстрого запуска можно создать alias.

Zsh:

```bash
echo "alias ao='\$HOME/repo/Ao/target/release/ao-tui'" >> ~/.zshrc
source ~/.zshrc
```

Bash:

```bash
echo "alias ao='\$HOME/repo/Ao/target/release/ao-tui'" >> ~/.bashrc
source ~/.bashrc
```

После этого Ao запускается командой:

```bash
ao
```

Alias сохраняется после перезагрузки системы.

## Background Services (Linux)

Для фоновых компонентов можно использовать systemd.

Пользовательские службы располагаются в:

```text
~/.config/systemd/user/
```

После создания или изменения службы:

```bash
systemctl --user daemon-reload
```

Основные команды:

```bash
systemctl --user start service-name
systemctl --user stop service-name
systemctl --user restart service-name
systemctl --user enable service-name
systemctl --user status service-name
```

Просмотр логов:

```bash
journalctl --user -u service-name -f
```

**Note:** Ao TUI является интерактивным приложением и требует полноценный терминал. Поэтому его не следует запускать как обычную фоновую службу systemd без настройки терминального окружения.

Для `llama-server` использование systemd подходит лучше.

## Configuration

Ao использует локальную конфигурацию для подключения к языковой модели и управления параметрами ассистента.

Рекомендуемое расположение пользовательских данных в Linux:

```text
~/.config/ao/       # Configuration
~/.local/share/ao/  # User data and history
~/.local/state/ao/  # Logs and runtime state
```

Это рекомендуемая схема организации данных; фактическая поддержка этих путей зависит от реализации приложения.

## Development

Ao активно разрабатывается. Архитектура, интерфейс и функциональность постепенно расширяются.

Основные направления разработки:

- Развитие модульной архитектуры.
- Улучшение управления контекстом.
- Работа с пользовательскими файлами.
- Развитие системы команд и инструментов.
- Интеграция голосового взаимодействия.
- Система памяти и истории диалогов.

## Status

🚧 **Early Development**

Ao is actively developed and its architecture is still evolving.

Некоторые функции находятся на стадии планирования или реализации.

## License

Open-source. License will be added as the project matures.
