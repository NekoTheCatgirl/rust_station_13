use bevy::prelude::*;
use bevy_quinnet::client::*;

pub fn start_client() -> AppExit {
    App::new()
        .add_plugins(QuinnetClientPlugin::default())
        .run()
}
