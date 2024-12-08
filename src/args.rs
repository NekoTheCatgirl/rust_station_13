use clap::{Parser, Subcommand};

#[derive(Debug, Clone, Copy, Parser)]
#[command(name = "rust_station_13")]
#[command(author = "Neko The Catgirl")]
#[command(about = "A rustified space station 13 build! Entirely cross platform!")]
pub struct ApplicationArguments {
    #[command(subcommand)]
    pub command: ApplicationCommands,
    
    /// If set, will enable debug log messages (only enable if something is going wrong)
    #[arg(short, long, default_value_t = false)]
    pub debug: bool,

    /// If set, the old logs will be cleared, this saves disk space.
    #[arg(long, default_value_t = false)]
    pub clear_logs: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Subcommand)]
pub enum ApplicationCommands {
    #[default]
    /// Tells the game that you wish to start it as a client
    Client,
    /// Tells the game that you wish to start it as a server
    Server,
    /// Tells the engine to uninstall all generated files and directories. Allowing easy deletion of the game after.
    Uninstall,
}