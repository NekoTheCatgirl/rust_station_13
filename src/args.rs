use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, Copy, Parser)]
#[command(name = "Rust Station 13")]
#[command(author = "HellFireNeko <johntheojacob@gmail.com>")]
#[command(about = "A rustified space station 13 build! Entirely cross platform!")]
pub struct ApplicationArguments {
    #[arg(short, long, default_value_t = true)]
    pub server_flag: bool,
    #[arg(long, value_enum, default_value_t = RenderBackend::Vulkan)]
    pub renderer: RenderBackend
}

#[derive(Clone, Copy, ValueEnum, Debug)]
pub enum RenderBackend {
    Vulkan,
    OpenGl,
    Metal,
}
