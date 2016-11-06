extern crate flexi_logger;

#[macro_use]
extern crate log;


#[test]
fn you_must_see_exactly_three_messages_above_1_err_1_warn_1_info() {
    flexi_logger::logger_options()
        .init(Some("info".to_string()))
        .unwrap();

    error!("This is an error message");
    warn!("This is a warning");
    info!("This is an info message");
    debug!("This is a debug message - you must not see it!");
    trace!("This is a trace message - you must not see it!");
}

use flexi_logger::{FlexiLogger, LogConfig, LogLevel};
#[allow(dead_code)]
fn ensure_visibility() {
    let fl = FlexiLogger::new(None, LogConfig::new()).unwrap();
    fl.fl_enabled(LogLevel::Error, "foo");
}