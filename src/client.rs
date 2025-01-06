use std::net::Ipv6Addr;

use bevy::prelude::*;
use bevy_quinnet::client::QuinnetClientPlugin;
use bevy_quinnet::{
    client::{
        certificate::CertificateVerificationMode, connection::ClientEndpointConfiguration,
        QuinnetClient,
    },
    shared::channels::ChannelsConfiguration,
};

use crate::{GameAssets, Player};

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(QuinnetClientPlugin::default())
            .add_systems(Startup, start_connection)
            .add_systems(Update, spawn_player_on_server_connect);
    }
}

/// Client can open multiple connections to multiple servers.
fn start_connection(mut client: ResMut<QuinnetClient>) {
    match client.open_connection(
        // You can specify binding ip and port. I think, it works like a filter,
        // and if we will put 0.0.0.0 and 0 port, then it will listen to any ip.
        ClientEndpointConfiguration::from_ips(Ipv6Addr::LOCALHOST, 6000, Ipv6Addr::UNSPECIFIED, 0),
        // We can check server certificate (helps to avoid dangerous sources)
        CertificateVerificationMode::SkipVerification,
        ChannelsConfiguration::default(),
    ) {
        Ok(local_id) => println!("Connecting... Local id: {}", local_id),
        Err(error) => println!("{}", error),
    };

    // When trully connected, you will receive a ConnectionEvent
}

fn spawn_player_on_server_connect(
    mut commands: Commands,
    mut connection_events_reader: EventReader<bevy_quinnet::client::connection::ConnectionEvent>,
) {
    for _ in connection_events_reader.read() {
        commands.spawn((
            Name::new("Player"),
            Player::default(),
            SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    ..default()
                },
                transform: Transform::from_scale(Vec3::new(30., 30., 1.)),
                ..default()
            },
        ));
    }
}
