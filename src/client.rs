use bevy::prelude::*;
use bevy_quinnet::client::*;

pub fn start_client(app: &mut App) -> &mut App {
    app.add_plugins(QuinnetClientPlugin::default())
        .add_systems(Startup, start_connection)
        .add_systems(Update, handle_server_messages)
}

fn start_connection(client: ResMut<QuinnetClient>) {
    // Handle connecting a client here
}

fn handle_server_messages(mut client: ResMut<QuinnetClient>) {
    while let Ok(Some(message)) = client.connection_mut().receive_message::<String>() {
        // String is just a template while the actuall server message enumeration is created.
    }
}
