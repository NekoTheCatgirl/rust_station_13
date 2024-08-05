use bevy::prelude::*;
use bevy_quinnet::server::*;

pub fn start_server() -> AppExit {
    App::new()
        .add_plugins(QuinnetServerPlugin::default())
        .run()
}
