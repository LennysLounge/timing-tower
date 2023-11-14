use std::{
    net::TcpStream,
    sync::{Arc, Mutex},
    thread,
};

use bevy::{
    ecs::schedule::IntoSystemConfigs,
    math::{vec2, vec3},
    prelude::{App, Res, ResMut, Resource, Startup, Update},
    render::color::Color,
    time::{Time, Timer, TimerMode},
    utils::info,
    DefaultPlugins,
};
use common::cell::style::CellStyleMessage;
use tracing::error;
use websocket::{sync::Client, Message, WebSocketError};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(RenderTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .insert_resource(WebsocketConnections {
            connections: Arc::new(Mutex::new(Vec::new())),
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (read_websockets, send_render_cell).chain())
        .run();
}

#[derive(Resource)]
struct WebsocketConnections {
    connections: Arc<Mutex<Vec<WebsocketClient>>>,
}

// impl WebsocketConnections {
//     fn send(&mut self, data: &[u8]) {
//         let message = Message::binary(data);
//         let mut connections = self.connections.lock().expect("Poisoned lock");
//         for connection in connections.iter_mut() {
//             if let Err(e) = connection.client.send_message(&message) {
//                 connection.connected = false;
//                 error!("Error trying to send on websocket: {e}")
//             }
//         }
//     }
// }

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

    let connections = websocket_writers.connections.clone();
    thread::spawn(move || {
        info("Starting websocket server");
        let server = websocket::sync::Server::bind("0.0.0.0:8001").unwrap();
        for connection in server.filter_map(Result::ok) {
            let client = connection.accept().unwrap();
            if let Err(e) = client.set_nonblocking(true) {
                error!("Error setting websocket to non blocking: {e}");
                continue;
            }
            let mut clients = connections.lock().expect("Poisoned lock");
            clients.push(WebsocketClient {
                connected: true,
                client,
            });
        }
    });
}

fn send_cell_style(client: &mut Client<TcpStream>) {
    let style = CellStyleMessage {
        text: String::from("Hello World"),
        text_color: Color::BLACK,
        text_size: 40.0,
        text_alignment: common::cell::style::TextAlignment::Center,
        text_position: vec2(0.0, 0.0),
        color: Color::Hsla {
            hue: rand::random::<f32>() * 360.0,
            saturation: rand::random::<f32>(),
            lightness: rand::random::<f32>(),
            alpha: 1.0,
        },
        pos: vec3(
            rand::random::<f32>() * 1180.0,
            rand::random::<f32>() * 620.0,
            rand::random::<f32>() * 1.0,
        ),
        size: vec2(
            rand::random::<f32>() * 80.0 + 20.0,
            rand::random::<f32>() * 80.0 + 20.0,
        ),
        skew: rand::random::<f32>() * 50.0 - 25.0,
        visible: true,
        rounding: [
            rand::random::<f32>() * 20.0,
            rand::random::<f32>() * 20.0,
            rand::random::<f32>() * 20.0,
            rand::random::<f32>() * 20.0,
        ],
    };

    let data = postcard::to_allocvec(&style).expect("Cannot convert to postcard");
    println!("data size: {}", data.len());
    client
        .send_message(&Message::binary(data))
        .expect("Cannot send");
}

fn read_websockets(websocket_connections: Res<WebsocketConnections>) {
    let mut clients = websocket_connections
        .connections
        .lock()
        .expect("Poisoned lock");
    for client in clients.iter_mut() {
        match client.client.recv_message() {
            Ok(m) => match m {
                websocket::OwnedMessage::Close(_) => client.connected = false,
                websocket::OwnedMessage::Binary(data) => {
                    let message = postcard::from_bytes::<common::websocket::Message>(&data)
                        .expect("Cannot deserialize");

                    match message {
                        common::websocket::Message::Opened => {
                            send_cell_style(&mut client.client);
                        }
                        common::websocket::Message::CellStyle(_) => (),
                    }
                }
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
        let mut clients = websocket_connections
            .connections
            .lock()
            .expect("Poisoned lock");
        for client in clients.iter_mut() {
            // if let Err(e) = client
            //     .client
            //     .send_message(&Message::text("Hello from server"))
            // {
            //     client.connected = false;
            //     error!("Error trying to send on websocket: {e}")
            // }
            send_cell_style(&mut client.client);
        }
        //println!("Render cell!");
    }
}
