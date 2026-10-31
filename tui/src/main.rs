use crossterm::{
    event::{KeyboardEnhancementFlags, PushKeyboardEnhancementFlags, PopKeyboardEnhancementFlags },
    execute,
};
use log::{Level, LevelFilter, Log, Metadata, Record};
use std::fs::{self, OpenOptions};
use std::io::{stdout, Write};
use std::path::PathBuf;
use std::sync::Mutex;

mod theme;

mod app;

mod input;

mod ui;

mod buffer;

use app::App;

struct FileLogger {
    file: Mutex<std::fs::File>,
}

impl Log for FileLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        if let Ok(mut file) = self.file.lock() {
            let _ = writeln!(
                file,
                "[{}] [{}] {} - {}",
                timestamp(),
                record.level(),
                record.target(),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}

fn timestamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    format!("{}.{:03}", now.as_secs(), now.subsec_millis())
}

fn init_file_logger() {
    let home = std::env::var_os("HOME").unwrap_or_else(|| "/tmp".into());
    let mut log_dir = PathBuf::from(home);
    log_dir.push(".local/state/ao");
    let _ = fs::create_dir_all(&log_dir);

    let mut log_path = log_dir;
    log_path.push("tui.log");

    // truncate on each run to avoid huge file
    if let Ok(file) = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&log_path)
    {
        let logger = FileLogger {
            file: Mutex::new(file),
        };
        if log::set_boxed_logger(Box::new(logger)).is_ok() {
            log::set_max_level(LevelFilter::Debug);
            log::info!("File logger started: {}", log_path.display());
        }
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    init_file_logger();

    execute!(
        stdout(),
        PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES)
    )?;
    let mut terminal = ratatui::init();

    let mut app = App::new();
    let app_result = app.run(&mut terminal).await;

    ratatui::restore();
    execute!(
        stdout(),
        PopKeyboardEnhancementFlags
    )?;
    app_result
}
