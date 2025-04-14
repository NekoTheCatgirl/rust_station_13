use std::{process::exit, thread::sleep, time::Duration};

use args::Arguments;
use clap::Parser;
use tracing::{debug, info, Level};

mod bevy_main;
mod args;
mod config;

#[allow(unused)]
fn main() {
    let args = Arguments::parse();
    
    // Configure logger.
    tracing_subscriber::fmt()
        .with_ansi(true)
        .with_thread_ids(true)
        .with_max_level(args.log_level)
        .init();

    // Provide a debug warning.
    debug!("Alert! Debug level is enabled. Log files may be excessively large. If this was unintended please ctrl+c right now before its too late, proceeding in 5s...");
    if args.log_level == Level::DEBUG {
        sleep(Duration::from_secs(5));
    }
    debug!("Proceeding despite warning, prepare for large log files.");

    let (command, port, ip, max_players, gui) = match args.command {
        args::MainCommand::Client => {
            info!("Starting up client!");
            let status = bevy_main::start_client();
            let status = match status {
                bevy::app::AppExit::Success => "Success".into(),
                bevy::app::AppExit::Error(non_zero) => format!("Error Code {non_zero}"),
            };
            debug!("App exited with {status}");
            exit(0); // Exit early to prevent the need to return the values
        },
        args::MainCommand::Server { command, port, ip, max_players, gui } => {
            debug!("Server selected, continue execution");
            (command, port, ip, max_players, gui)
        },
    };

    match command {
        args::ServerCommands::Create { name, description } => {
            info!("Preparing server creation");
            // Check if the server already exists...
        },
        args::ServerCommands::Start { server } => {
            info!("Looking for server...");
            // Check if the server exists...
            info!("Found server! Starting up server");
        },
        args::ServerCommands::Wipe { server } => {
            info!("Looking for server...");
            // Check if the server exists...
            info!("Found server! Wiping persistant data");
        },
        args::ServerCommands::ConfigureDB { for_server, address, port, user, password, database } => {
            info!("Looking for server...");
            // Check if the server exists...
            info!("Found server! Preparing to configure database for the server.");
        },
    }

}
