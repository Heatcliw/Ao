# Ao (アオ)

**Ao** — open-source локальный ИИ-ассистент, написанный на **Rust** и работающий с локальными языковыми моделями.

Проект создаётся как модульный ИИ-агент, который может работать с контекстом пользователя, файлами и внешними инструментами, сохраняя основную логику и данные локально.

## Features

* 🧠 Local LLM
* 👤 Custom Profile & Personality
* ⚙️ Dynamic System Prompt
* 📁 File Context
* 🧩 Modular architecture
* 🌐 Web Research — planned
* 🎙️ Voice interaction — planned

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

## Tech Stack

* Rust
* Tokio
* Reqwest
* Serde
* Local LLM / llama-server

## Status

🚧 **Early development**

Ao is actively developed and its architecture is still evolving.

## License

Open-source. License will be added as the project matures.
