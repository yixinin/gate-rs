extern crate futures;
extern crate grpcio;
extern crate libc;
extern crate protobuf;
extern crate ws;

extern crate crossbeam;
#[macro_use]
extern crate crossbeam_channel;

use crossbeam_channel as channel;
use session::gate::Gate;
use std::thread;

use ws::Builder;
use ws::Settings;

use session::ChannelMessage;

use session::center_state::CenterState;
use session::client_state::ClientState;
use session::room_state::RoomState;

use session::connection::Connection;

use config::AppConfig;

pub mod config;
pub mod protocol;
pub mod session;
fn main() {
    let (sender, receiver) = channel::unbounded::<ChannelMessage>();

    thread::spawn(move || {
        Gate {
            client_state: ClientState::default(),
            room_state: RoomState::new(),
            center_state: CenterState::new(),
            config: AppConfig::with_config_file("config.toml"),
        }
        .listen(receiver);
    });
    let settings = Settings {
        max_connections: 5000,
        tcp_nodelay: true,
        ..Settings::default()
    };

    let socket = Builder::new()
        .with_settings(settings)
        .build(move |ws| Connection {
            ws,
            room_channel: sender.clone(),
        })
        .expect("Panicking on WebSocket build!");
    socket
        .listen("localhost:8081")
        .expect("Panicking on WebSocket listen!");
}
