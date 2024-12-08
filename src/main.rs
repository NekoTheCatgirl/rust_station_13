use std::{
    fs,
    process::exit, sync::atomic::{AtomicU16, Ordering},
};

use args::ApplicationCommands;
use bevy::prelude::*;
use clap::Parser;
use log::{error, LevelFilter};

use crate::logger::setup_logger;

mod args;
mod game_code;
mod prestart;
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

    if args.clear_logs {
        let _ = fs::remove_dir_all("logs");
    }

    let _ = fs::create_dir_all("logs");
    
    setup_logger(if args.debug {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    });
    
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

    if args.command == ApplicationCommands::Uninstall {
        // Got it, proceed to remove all folders relating to the game! Including log files
        info!("Uninstalling game, this may take a bit depending on how many assets were downloaded!");
        
        exit(0);
    }

    if prestart::prestart_networkcheck().is_err() {
        error!("Network error detected! Could not ping google.com (pre-network test)");
        exit(3)
    }

    let mut app = App::new();

    match args.command {
        ApplicationCommands::Client => {
            info!("Adding client code!");
        },
        ApplicationCommands::Server => {
            info!("Adding server code!")
        },
        ApplicationCommands::Uninstall => {
            error!("Unknown error, how did we get here?")
        }
    }

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
