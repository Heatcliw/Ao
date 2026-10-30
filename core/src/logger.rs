use std::str::FromStr;

/// ### Описание
/// Уровни логирования Ao.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeLogs {
    Debug,
    Info,
    Error,
    Warn,
    Trace,
    None,
}

impl TypeLogs {
    /// ### Описание
    /// Возвращает человекочитаемое строковое представление уровня логирования.
    pub fn as_str(self) -> &'static str {
        match self {
            TypeLogs::Debug => "debug",
            TypeLogs::Error => "error",
            TypeLogs::Info => "info",
            TypeLogs::Warn => "warn",
            TypeLogs::Trace => "trace",
            TypeLogs::None => "off",
        }
    }

    /// ### Описание
    /// Парсит строку с уровнем логирования в [TypeLogs].
    ///
    /// ### Пример
    /// ```text
    /// "info"   -> TypeLogs::Info
    /// "DEBUG"  -> TypeLogs::Debug
    /// "warn"   -> TypeLogs::Warn
    /// "[INFO]" -> TypeLogs::Info
    /// ```
    pub fn parse(value: &str) -> Option<Self> {
        let normalized = value
            .trim()
            .trim_start_matches('[')
            .trim_end_matches(']')
            .trim()
            .to_lowercase();

        match normalized.as_str() {
            "debug" => Some(TypeLogs::Debug),
            "info" => Some(TypeLogs::Info),
            "error" => Some(TypeLogs::Error),
            "warn" | "warning" => Some(TypeLogs::Warn),
            "trace" => Some(TypeLogs::Trace),
            "off" | "none" => Some(TypeLogs::None),
            _ => None,
        }
    }
}

impl FromStr for TypeLogs {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value).ok_or_else(|| format!("Unknown log type: {value}"))
    }
}

/// ### Описание
/// Парсит уровень лога из произвольной строки.
///
/// ### Пример
/// ```text
/// parse_logs("Info")  -> Some(TypeLogs::Info)
/// parse_logs("fatal") -> None
/// ```
pub fn parse_logs(value: &str) -> Option<TypeLogs> {
    TypeLogs::parse(value)
}

/// ### Описание
/// Инициализирует логгер Ao с заданным уровнем.
pub fn build_logger(level: TypeLogs) {
    let env = env_logger::Env::default().default_filter_or(level.as_str());
    let _ = env_logger::Builder::from_env(env).init();
}
