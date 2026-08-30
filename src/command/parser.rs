
/// ### Описание
/// Основной тип команд Ao, объединяющий все категории команд.
pub enum Command {
    App(AppCommand),

    File(FileCommand),
    Profile(ProfileCommand),
    Server(ServerCommand),
    Request(RequestCommand),
    Memory(MemoryCommand),

    /// ### Описание
    /// Представляет неизвестную или неподдерживаемую команду.
    /// В этом случае введённый текст обрабатывается как обычный текст запроса.
    ///
    /// ### Пример
    /// ```text
    /// :unknown-command
    /// ```
    Unknown,
}
/// ### Описание
/// Основные команды управления приложением Ao.
pub enum AppCommand {
    /// ### Описание
    /// Завершает работу Ao.
    ///
    /// ### Пример
    /// ```text
    /// :exit
    /// ```
    Exit,

    /// ### Описание
    /// Добавляет текст в текущий запрос.
    ///
    /// ### Пример
    /// ```text
    /// Прочитай этот код и найди ошибку.
    /// :push
    /// ```
    Push,

    /// ### Описание
    /// Отправляет текущий запрос локальной LLM.
    ///
    /// ### Пример
    /// ```text
    /// :send
    /// ```
    Send,
}


/// ### Описание
/// Команды для работы с файлами и директориями.
pub enum FileCommand {
    /// ### Описание
    /// Читает указанный файл или директорию и добавляет
    /// содержимое прочитанных файлов в текущий запрос.
    ///
    /// ### Пример
    /// ```text
    /// :file-add ./src/main.rs
    /// ```
    Add(String),
    /// ### Описание
    /// Добавляет путь в список игнорируемых файлов и директорий.
    /// Игнорируемые пути не добавляются в запрос при чтении файлов.
    ///
    /// ### Пример
    /// ```text
    /// :file-ignore ./target
    /// ```
    Ignore(String),

}

/// ### Описание
/// Команды для настройки профиля и поведения Ao.
pub enum ProfileCommand {
    /// ### Описание
    /// Устанавливает идентичность Ao.
    ///
    /// ### Пример
    /// ```text
    /// :iden Ты — Ao, локальный ИИ-ассистент.
    /// ```
    Identity(String),

    /// ### Описание
    /// Устанавливает личность и характер Ao.
    ///
    /// ### Пример
    /// ```text
    /// :pers Спокойная, внимательная и рациональная.
    /// ```
    Personality(String),

    /// ### Описание
    /// Добавляет правило поведения Ao.
    ///
    /// ### Пример
    /// ```text
    /// :rule Всегда объясняй причину найденной ошибки.
    /// ```
    Rule(String),

    /// ### Описание
    /// Добавляет правило стиля ответа Ao.
    ///
    /// ### Пример
    /// ```text
    /// :style Отвечай кратко и по существу.
    /// ```
    Style(String),

}


/// ### Описание
/// Команды для управления llama-server и его параметрами.
pub enum ServerCommand {
    /// ### Описание
    /// Запускает llama-server с текущими настройками.
    ///
    /// ### Пример
    /// ```text
    /// :server-start
    /// ```
    Start,
    /// ### Описание
    /// Останавливает запущенный llama-server.
    ///
    /// ### Пример
    /// ```text
    /// :server-stop
    /// ```
    Stop,
    /// ### Описание
    /// Изменяет размер контекста llama-server.
    ///
    /// По умолчанию: 4096.
    ///
    /// Изменение применяется при следующем запуске сервера.
    ///
    /// ### Пример
    /// ```text
    /// :server-stop
    /// :server-context 8192
    /// :server-start
    /// ```
    Context(usize),
    /// ### Описание
    /// Изменяет количество GPU-слоёв llama-server.
    ///
    /// По умолчанию: 20.
    ///
    /// Изменение применяется при следующем запуске сервера.
    ///
    /// ### Пример
    /// ```text
    /// :server-stop
    /// :server-gpu-layers 15
    /// :server-start
    /// ```
    GPULayers(usize),
}

/// ### Описание
/// Команды для управления параметрами текущего запроса.
pub enum RequestCommand {
    /// ### Описание
    /// Изменяет максимальное количество генерируемых токенов запроса.
    ///
    /// По умолчанию: 512.
    ///
    /// Изменение применяется при следующем запросе `:send`.
    ///
    /// ### Пример
    /// ```text
    /// :max-tokens 1024
    /// ```
    MaxTokens(usize),
}

/// ### Описание
/// Команды для управления памятью Ao.
pub enum MemoryCommand {
    
}

pub fn check_command(input: &str) -> Command {
    
    if let Some(value) = input.strip_prefix(":identity  ") {
        return Command::Profile(ProfileCommand::Identity(value.to_string()));
    }

    if let Some(value) = input.strip_prefix(":personality  ") {
        return Command::Profile(ProfileCommand::Personality(value.to_string()));
    }

    if let Some(value) = input.strip_prefix(":rule ") {
        return Command::Profile(ProfileCommand::Rule(value.to_string()));
    }

    if let Some(value) = input.strip_prefix(":style ") {
        return Command::Profile(ProfileCommand::Style(value.to_string()));
    }

    if let Some(path) = input.strip_prefix(":file-add ") {
        return Command::File(FileCommand::Add(path.to_string()));
    }

    if let Some(path) = input.strip_prefix(":file-ignore ") {
        return Command::File(FileCommand::Ignore(path.to_string()));
    }

    if let Some(tokens) = input.strip_prefix(":max-tokens ") {
        if let Ok(tokens) = tokens.parse::<usize>() {
            return Command::Request(RequestCommand::MaxTokens(tokens));
        }
    }

    if let Some(context) = input.strip_prefix(":server-context ") {
        if let Ok(context) = context.parse::<usize>() {
            return Command::Server(ServerCommand::Context(context));
        }
    }

    if let Some(layers) = input.strip_prefix(":server-gpu-layers ") {
        if let Ok(layers) = layers.parse::<usize>() {
            return Command::Server(ServerCommand::GPULayers(layers));
        }
    }

    match input.trim() {
        ":server-start" => Command::Server(ServerCommand::Start),
        ":server-stop" => Command::Server(ServerCommand::Stop),

        ":exit" => Command::App(AppCommand::Exit),
        ":push" => Command::App(AppCommand::Push),
        ":send" => Command::App(AppCommand::Send),

        _ => Command::Unknown,
    }
}