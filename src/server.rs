use std::net::Ipv6Addr;

use bevy::prelude::*;
use bevy_quinnet::{
    server::{
        certificate::CertificateRetrievalMode, QuinnetServer, QuinnetServerPlugin,
        ServerEndpointConfiguration,
    },
    shared::channels::ChannelsConfiguration,
};

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(QuinnetServerPlugin::default())
            .add_systems(Startup, start_listening);
    }
}

fn start_listening(mut server: ResMut<QuinnetServer>) {
    match server.start_endpoint(
        ServerEndpointConfiguration::from_ip(Ipv6Addr::UNSPECIFIED, 6000),
        CertificateRetrievalMode::GenerateSelfSigned {
            server_hostname: "multiplayer_test".to_string(),
        },
        ChannelsConfiguration::default(),
    ) {
        Ok(_) => info!("Server started!"),
        Err(error) => println!("{}", error),
    };
}
