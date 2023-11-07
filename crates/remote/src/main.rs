use std::thread;

use bevy::{
    prelude::{App, Startup},
    utils::info,
    DefaultPlugins,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup() {
    thread::spawn(|| {
        info("Starting web server");
        rouille::start_server("0.0.0.0:8000", move |request| {
            println!("Requested: {}", request.url());
            rouille::match_assets(&request, concat!(file!(), "/../../assets"))
        });
    });
    thread::spawn(|| {
        info("Starting websocket server");
        let server = websocket::sync::Server::bind("0.0.0.0:8001").unwrap();
        for connection in server.filter_map(Result::ok) {
            thread::spawn(move || {
                let mut client = connection.accept().unwrap();
                loop {
                    match client.recv_message() {
                        Ok(m) => println!("Incomming message: {:?}", m),
                        Err(_) => break,
                    }
                }
            });
        }
    });
}
