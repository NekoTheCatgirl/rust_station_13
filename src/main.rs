use std::{
    fs::{self, File},
    process::exit, sync::atomic::{AtomicU16, Ordering},
};

use bevy::app::AppExit;
use chrono::Local;
use clap::Parser;
use fern::Dispatch;
use log::{debug, error, LevelFilter};

mod args;
mod client;
mod game_code;
mod prestart;
mod server;

// Start the game exit error as 0
// If a error occurs during gameplay and the game is forced to exit, return this and use exit with this code.
pub static GAME_EXIT_ERROR: AtomicU16 = AtomicU16::new(0);

fn setup_logger(level: LevelFilter) {
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
        .chain(File::create(&log_file_name).expect("Failed to create log file"));

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

fn main() {
    let args = args::ApplicationArguments::parse();

    if args.uninstall {
        // Got it, proceed to remove all folders relating to the game! Including log files
    }

    if args.clear_logs {
        let _ = fs::remove_dir_all("logs");
    }

    let _ = fs::create_dir_all("logs");

    setup_logger(if args.log_debug {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    });

    if let Err(_) = prestart::prestart_networkcheck() {
        error!("Network error detected! Could not ping google.com (prenetwork test)");
        exit(3)
    }

    let exit_code = if args.server_flag {
        // Check for server dirs.

        // Bootup the server system.
        server::start_server()
    } else {
        // Check for client dirs.
        if prestart::dirchecks::client_dirs_exist() != true {
            // Generate client dirs.
        }
        // Bootup the client system.
        client::start_client()
    };

    let code = GAME_EXIT_ERROR.load(Ordering::SeqCst);
    if code != 0 {
        error!("Application exited with error code {code}! Reffer to https://github.com/HellFireNeko/rust_station_13 for information!");
        exit(code as i32);
    }

    match exit_code {
        AppExit::Success => exit(0),
        AppExit::Error(_) => exit(2),
    }
}
