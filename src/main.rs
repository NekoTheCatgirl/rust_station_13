use clap::Parser;

mod args;

fn main() {
    let args = args::ApplicationArguments::parse();

    if args.server_flag {
        // Bootup the server system.
    } else {
        // Bootup the client system.
    }
}
