use std::{
    fs,
    process::exit,
    sync::atomic::{AtomicU16, Ordering},
};

use args::ApplicationCommands;
use bevy::{
    log::LogPlugin, prelude::*
};
use clap::Parser;
use log::{error, LevelFilter};
use prestart::{dirchecks::{client_dirs_exist, server_dirs_exist}, dirgen::{client_gen, server_gen}, dirrem};

use crate::logger::setup_logger;

mod args;
mod errors;
mod logger;
mod prestart;

/// Start the game exit error as 0
/// If a error occurs during gameplay and the game is forced to exit, return this and use exit with this code.
pub static GAME_EXIT_ERROR: AtomicU16 = AtomicU16::new(0);

struct ConfiguredDefaultPlugins;

impl PluginGroup for ConfiguredDefaultPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        DefaultPlugins
            .build()
            .set(bevy::render::RenderPlugin {
                render_creation: bevy::render::settings::RenderCreation::Automatic(bevy::render::settings::WgpuSettings {
                    backends: Some(bevy::render::settings::Backends::VULKAN),
                    ..Default::default()
                }),
                synchronous_pipeline_compilation: false,
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Rust station 13".to_owned(),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .disable::<LogPlugin>()
    }
}

fn main() {
    let args = args::ApplicationArguments::parse();

    if args.clear_logs {
        let _ = fs::remove_dir_all("logs");
    }

    let _ = fs::create_dir_all("logs");

    setup_logger(if args.debug {
        LevelFilter::Debug
    } else {
        LevelFilter::Warn
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
        info!(
            "Uninstalling game, this may take a bit depending on how many assets were downloaded!"
        );

        dirrem::dir_cleanup();

        exit(0);
    }

    if prestart::prestart_networkcheck().is_err() {
        error!("Network error detected! Could not ping google.com (pre-network test)");
        exit(3)
    }

    let mut app = App::new();

    app.add_plugins(ConfiguredDefaultPlugins);

    match args.command {
        ApplicationCommands::Client => {
            info!("Checking configs...");
            if client_dirs_exist() {

            } else {
                info!("No config found, generating new configs...");
                client_gen();
            }
            info!("Adding client code!");
        }
        ApplicationCommands::Server => {
            info!("Checking configs...");
            if server_dirs_exist() {

            } else {
                info!("No config found, generating new configs...");
                server_gen();
            }
            info!("Adding server code!");
        }
        ApplicationCommands::Uninstall => {
            error!("Unknown error, how did we get here?")
        }
    }

    let exit_code = app.run();
    info!("Game exit recieved");

    let code = GAME_EXIT_ERROR.load(Ordering::SeqCst);
    if code != 0 {
        exit(errors::log_error_code(code));
    }

    match exit_code {
        AppExit::Success => exit(0),
        AppExit::Error(_) => exit(2),
    }
}
