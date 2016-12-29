extern crate log;

use log::{LogRecord, LogLevelFilter, LogMetadata, SetLoggerError};

struct SimpleLogger {
    filter: LogLevelFilter,
}

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= self.filter
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }
}

pub fn init(filter: LogLevelFilter) -> Result<(), SetLoggerError> {
    log::set_logger(|max_log_level| {
        max_log_level.set(filter);
        let logger = SimpleLogger {
            filter: filter,
        };
        Box::new(logger)
    })
}
