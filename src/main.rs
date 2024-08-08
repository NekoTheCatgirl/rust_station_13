use std::{
    fs,
    process::exit, sync::atomic::{AtomicU16, Ordering},
};

use bevy::prelude::*;
use clap::Parser;
use log::{error, LevelFilter};

use crate::logger::setup_logger;

mod args;
mod client;
mod game_code;
mod prestart;
mod server;
mod fileutils;
mod errors;
mod logger;
mod macros;
mod message;

/// Start the game exit error as 0
/// If a error occurs during gameplay and the game is forced to exit, return this and use exit with this code.
pub static GAME_EXIT_ERROR: AtomicU16 = AtomicU16::new(0);

fn main() {
    let args = args::ApplicationArguments::parse();
    
    match sys_info::mem_info() {
        Ok(mem) => {
            if mem.total < 1_048_576 {
                exit(errors::log_error_code(13));
            }
        }
        Err(_) => {
            error!("Failed to retrieve system memory information!");
            exit(1);
        }
    }

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

    if prestart::prestart_networkcheck().is_err() {
        error!("Network error detected! Could not ping google.com (prenetwork test)");
        exit(3)
    }

    let mut app = App::new();

    if args.server_flag {
        // Check for server dirs.
        if !prestart::dirchecks::server_dirs_exist() {
            // Generate server dirs.
        }
        // Bootup the server system.
        server::start_server(&mut app);
    } else {
        // Check for client dirs.
        if !prestart::dirchecks::client_dirs_exist() {
            // Generate client dirs.
        }
        // Bootup the client system.
        client::start_client(&mut app);
    }

    // Add all the game systems (both sides need them to keep in sync)

    let exit_code = app.run();

    let code = GAME_EXIT_ERROR.load(Ordering::SeqCst);
    if code != 0 {
        exit(errors::log_error_code(code));
    }

    match exit_code {
        AppExit::Success => exit(0),
        AppExit::Error(_) => exit(2),
    }
}
