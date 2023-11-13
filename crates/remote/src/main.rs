use std::{
    net::TcpStream,
    sync::{Arc, Mutex},
    thread,
};

use bevy::{
    prelude::{App, IntoSystemConfigs, Res, ResMut, Resource, Startup, Update},
    time::{Time, Timer, TimerMode},
    utils::info,
    DefaultPlugins,
};
use common::test::CellStyle;
use tracing::error;
use websocket::{sync::Client, Message, WebSocketError};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .insert_resource(RenderTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .insert_resource(WebsocketConnections {
            connections: Arc::new(Mutex::new(Vec::new())),
        })
        .add_systems(
            Update,
            (update_websocket_connection, send_render_cell).chain(),
        )
        .run();
}

#[derive(Resource)]
struct WebsocketConnections {
    connections: Arc<Mutex<Vec<WebsocketClient>>>,
}
struct WebsocketClient {
    connected: bool,
    client: Client<TcpStream>,
}

fn setup(websocket_writers: ResMut<WebsocketConnections>) {
    thread::spawn(|| {
        info("Starting web server");
        rouille::start_server("0.0.0.0:8000", move |request| {
            println!("Requested: {}", request.url());
            rouille::match_assets(&request, concat!(file!(), "/../../web"))
        });
    });

    let writers = websocket_writers.connections.clone();
    thread::spawn(move || {
        info("Starting websocket server");
        let server = websocket::sync::Server::bind("0.0.0.0:8001").unwrap();
        for connection in server.filter_map(Result::ok) {
            let client = connection.accept().unwrap();
            if let Err(e) = client.set_nonblocking(true) {
                error!("Error setting websocket to non blocking: {e}");
                continue;
            }

            let mut clients = writers.lock().expect("Poisoned lock");
            clients.push(WebsocketClient {
                connected: true,
                client,
            });
        }
    });
}

fn update_websocket_connection(websocket_connections: Res<WebsocketConnections>) {
    let mut clients = websocket_connections
        .connections
        .lock()
        .expect("Poisoned lock");
    for client in clients.iter_mut() {
        match client.client.recv_message() {
            Ok(m) => match m {
                websocket::OwnedMessage::Close(_) => client.connected = false,
                m @ _ => println!("Incomming message: {m:?}"),
            },
            Err(WebSocketError::IoError(ref e)) if e.kind() == std::io::ErrorKind::WouldBlock => (),
            Err(e) => {
                error!("Error in websocket: {e:?}");
                client.connected = false;
            }
        };
    }
    clients.retain(|c| c.connected);
}

#[derive(Resource)]
struct RenderTimer(Timer);

fn send_render_cell(
    time: Res<Time>,
    mut render_timer: ResMut<RenderTimer>,
    websocket_connections: Res<WebsocketConnections>,
) {
    if render_timer.0.tick(time.delta()).just_finished() {
        let cell_style = CellStyle {
            message: String::from("Hello from server"),
            number: time.elapsed_seconds() as i32,
        };
        let cell_style = postcard::to_allocvec(&cell_style).expect("Cannot serialize");

        let mut clients = websocket_connections
            .connections
            .lock()
            .expect("Poisoned lock");
        for client in clients.iter_mut() {
            if let Err(e) = client
                .client
                .send_message(&Message::binary(cell_style.clone()))
            {
                client.connected = false;
                error!("Error trying to send on websocket: {e}")
            }
        }
        println!("Render cell!");
    }
}
