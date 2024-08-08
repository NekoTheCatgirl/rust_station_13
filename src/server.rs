use bevy::prelude::*;
use bevy_quinnet::server::*;

pub fn start_server(app: &mut App) -> &mut App {
    app.add_plugins(QuinnetServerPlugin::default())
}
