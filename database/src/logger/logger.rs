#[derive(PartialEq, PartialOrd)]
pub enum LogLevel {
    Error,
    Warning,
    Info,
}

pub struct Logger {
    log_level: LogLevel,
}

impl Logger {
    pub fn new(log_level: LogLevel) -> Self {
        Logger { log_level }
    }

    pub fn log(&self, lvl: LogLevel, msg: String) {
        if lvl <= self.log_level {
            println!("[{}] {}", self.level_to_str(lvl), msg);
        }
    }

    fn level_to_str(&self, level: LogLevel) -> String {
        match level {
            LogLevel::Info => "I".to_string(),
            LogLevel::Warning => "W".to_string(),
            LogLevel::Error => "E".to_string(),
        }
    }

    pub fn set_new_level(&mut self, lvl: LogLevel) {
        self.log_level = lvl;
    }

    pub fn info(&self, message: String) {
        self.log(LogLevel::Info, message);
    }

    pub fn warning(&self, message: String) {
        self.log(LogLevel::Warning, message);
    }

    pub fn error(&self, message: String) {
        self.log(LogLevel::Error, message);
    }
}
