use std::fs::File;
use chrono::Local;
use fern::Dispatch;
use log::{debug, LevelFilter};

pub fn setup_logger(level: LevelFilter) {
    let log_file_name = format!(
        "logs/logfile-{}.log",
        Local::now().format("%Y-%m-%d_%H-%M-%S")
    );

    let file_logger = Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}] [{}] => {}",
                Local::now().format("%Y-%m-%d_%H-%M-%S"),
                record.level(),
                message,
            ))
        })
        .level(level)
        .chain(File::create(log_file_name).expect("Failed to create log file"));

    let stdout_logger = Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}] [{}] => {}",
                Local::now().format("%Y-%m-%d_%H-%M-%S"),
                record.level(),
                message,
            ))
        })
        .level(level)
        .chain(std::io::stdout());

    let combined_dispatch = Dispatch::new().chain(file_logger).chain(stdout_logger);

    combined_dispatch
        .apply()
        .expect("Failed to set up global logger!");

    debug!("Debug logging is enabled, be warned this can lead to increase log file sizes!!!");
}
