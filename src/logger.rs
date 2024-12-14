use chrono::Local;
use colored::*;
use fern::Dispatch;
use log::{debug, Level, LevelFilter};

pub fn setup_logger(level: LevelFilter) {
    
    let log_file_name = format!(
        "logs/logfile-{}.log",
        Local::now().format("%Y-%m-%d_%H-%M-%S")
    );

    // The logger setup is done synchronously to initialize it, then we use async tasks for logging.
    let file_logger = Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}] [{}] [{}] => {}",
                Local::now().format("%Y-%m-%d_%H-%M-%S"),
                record.level(),
                record.module_path().unwrap_or("unknown"),
                message,
            ))
        })
        .level(level)
        .level_for("rust_station_13", LevelFilter::Info)
        .chain(fern::log_file(&log_file_name).unwrap())
        .into_shared();

    let stdout_logger = Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}] [{}] [{}] => {}",
                format!("{}", Local::now().format("%Y-%m-%d_%H-%M-%S")).bright_black(),
                record_color(record.level()),
                record.module_path().unwrap_or("unknown").bright_black(),
                message,
            ))
        })
        .level(level)
        .level_for("rust_station_13", LevelFilter::Info)
        .chain(std::io::stdout())
        .into_shared();

    let combined_dispatch = Dispatch::new().chain(file_logger).chain(stdout_logger);

    combined_dispatch
        .apply()
        .expect("Failed to set up global logger!");

    debug!("Debug logging is enabled, be warned this can lead to increased log file sizes!!!");
}

fn record_color(level: Level) -> ColoredString {
    match level {
        Level::Error => "ERROR".red(),
        Level::Warn => "WARN".yellow(),
        Level::Info => "INFO".green(),
        Level::Debug => "DEBUG".purple(),
        Level::Trace => "TRACE".blue(),
    }
}
