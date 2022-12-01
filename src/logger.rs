use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

pub struct Logger {
    message: Vec<LogEntryMessage>
}

enum LogEntryMessage {
    Error(String),
    Info(String),
    Warn(String)
}

impl Logger {
    pub fn new() -> Self {
        Self {
            message: Vec::new()
        }
    }

    pub fn info(&mut self, message: &str) {
        let message = Logger::message("INFO", message);
        self.message.push(LogEntryMessage::Info(message.clone()));
        println!("\x1b[92m{message}\x1b[0m");
    }

    pub fn warn(&mut self, message: &str) {
        let message = Logger::message("WARN", message);
        self.message.push(LogEntryMessage::Error(message.clone()));
        println!("\x1b[93m{message}\x1b[0m");
    }

    pub fn error(&mut self, message: &str) { 
        let message = Logger::message("ERR", message);
        self.message.push(LogEntryMessage::Warn(message.clone()));
        eprintln!("\x1b[91m{message}\x1b[0m");
    }

    fn message(prefix: &str, message: &str) -> String {
        let time = Logger::time();
        format!("{prefix} | {time} - {message}")
    }

    fn time() -> String {
        let format: String = OffsetDateTime::now_utc().format(&Rfc3339).unwrap();
        format
    }
}
