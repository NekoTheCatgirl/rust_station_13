# Rust station 13
A rustified ss13 clone!

## Discord
Rust station 13 has a development and game discord, join [here!](https://discord.gg/pnvyqBAT2e)

## Table of contents
* [Credits](#credits)
* [Directories](#directories)
* [Build from source](#build-from-source)
* [Hosting a server](#hosting-a-server)
* [License](#license)

## Credits
Currently there are no credits aside from SS13, being the inspiration for the entire project!

## Directories
The games home directory based on the plaform you are using are the following:
* Linux: `/home/{user}/.games/rust_station_13/`
* Windows: `C:\Users\{user}\Documents\My Games\Rust Station 13\`
* Mac: `/Users/{user}/.games/rust_station_13/`

## Build from source
In the event that the current latest build has not yet been compiled and uploaded for your operating system, you can simply follow these steps in order to have the latest git build:

1. Clone the repository `git clone https://github.com/NekoTheCatgirl/rust_station_13`
2. Execute the build process `cargo build --release` alt `cargo run --release -- client` / `cargo run --release -- server`

## Hosting a server
Hosting a server is easily done, its baked into the main application. However there are some prerequisites if you wish to use certain features.

* [Host without databse](#start-a-server-without-database)
* [Host with databse](#start-a-server-with-a-database)

### Start a server without database:
1. Navigate to the executable
2. Run `rust_station_13 server` once to generate all the configuration
3. Navigate to the `server_config` directory in the game home directory refferenced in [Directories](#directories)
4. Open up `host.toml` and configure the port you wish to use (default is `2323`)
5. Port forward the desired port or use a tunneling service like [Playit](https://playit.gg/)
6. Run the `rust_station_13 server` again and now you can connect using the client

### Start a server with a database:
WORK IN PROGRESS NO DATABASE YET

## License
All code is licensed under APGL 3.0
