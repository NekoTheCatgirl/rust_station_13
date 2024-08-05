use clap::Parser;

#[derive(Debug, Clone, Copy, Parser)]
#[command(name = "rust_station_13")]
#[command(author = "HellFireNeko <johntheojacob@gmail.com>")]
#[command(about = "A rustified space station 13 build! Entirely cross platform!")]
pub struct ApplicationArguments {
    /// If set, will start a server instead of the client
    #[arg(long, default_value_t = false)]
    pub server_flag: bool,
    
    /// If set, will enable debug log messages (only enable if something is going wrong)
    #[arg(long, default_value_t = false)]
    pub log_debug: bool,

    /// If set, the old logs will be cleared, this saves disk space.
    #[arg(long, default_value_t = false)]
    pub clear_logs: bool,

    /// If set, this will uninstall all the assets and after you can safely remove the executable from your system.
    #[arg(long, default_value_t = false)]
    pub uninstall: bool,
}