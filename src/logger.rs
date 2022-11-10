use std::fmt::Debug;

use time::{OffsetDateTime, PrimitiveDateTime};

pub struct Logger {
    message: Vec<LogEntry>
}

enum LogEntry {
    ErrorMessage(String),
    InfoMessage(String),
    WarnMessage(String)
}

impl Logger {
    pub fn new() -> Self {
        Self {
            message: Vec::new()
        }
    }

    pub fn info(&mut self, message: &str) {
        let message = Logger::message("INFO", message);
        self.message.push(LogEntry::InfoMessage(message.clone()));
        println!("\x1b[92m{message}\x1b[0m");
    }

    pub fn warn(&mut self, message: &str) {
        let message = Logger::message("ERR", message);
        self.message.push(LogEntry::ErrorMessage(message.clone()));
        println!("\x1b[93m{message}\x1b[0m");
    }

    pub fn error(&mut self, message: &str) { 
        let message = Logger::message("WARN", message);
        self.message.push(LogEntry::WarnMessage(message.clone()));
        eprintln!("\x1b[91m{message}\x1b[0m");
    }

    fn message(prefix: &str, message: &str) -> String {
        let time = Logger::time();
        format!("{prefix} | {time} - {message}")
    }

    fn time() -> PrimitiveDateTime {
        let time = OffsetDateTime::now_utc();
        PrimitiveDateTime::new(time.date(), time.time())
    }
}