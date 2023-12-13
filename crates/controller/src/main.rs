use std::env;

use backend::{
    savefile::{Savefile, SavefileChanged},
    BackendPlugin,
};
use bevy::{
    app::Startup,
    ecs::{event::EventWriter, system::Query},
    math::{vec2, vec3},
    prelude::{App, Res, ResMut, Resource, Update},
    render::color::Color,
    time::{Time, Timer, TimerMode},
    DefaultPlugins,
};
use common::communication::{CellStyle, ToRendererMessage};
use webserver::WebserverPlugin;
use websocket::{ClientState, WebsocketClient, WebsocketPlugin};

mod webserver;
mod websocket;

fn main() {
    println!("pwd: {:?}", env::current_dir());
    App::new()
        .add_plugins(BackendPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugins(WebsocketPlugin)
        .add_plugins(WebserverPlugin)
        .insert_resource(RenderTimer(Timer::from_seconds(
            0.001,
            TimerMode::Repeating,
        )))
        .add_systems(Startup, load_savefile)
        //        .add_systems(Update, send_render_cell)
        .run();
}

#[derive(Resource)]
struct RenderTimer(Timer);

fn load_savefile(
    mut savefile: ResMut<Savefile>,
    savefile_changed_event: EventWriter<SavefileChanged>,
) {
    savefile.load("../../savefile/style.style.json", savefile_changed_event);
}

fn send_render_cell(
    _time: Res<Time>,
    mut _render_timer: ResMut<RenderTimer>,
    mut clients: Query<&mut WebsocketClient>,
) {
    if !_render_timer.0.tick(_time.delta()).just_finished() {
        return;
    }

    let styles = get_cell_styles();

    for mut client in clients.iter_mut() {
        if client.state() == &ClientState::Ready {
            client.send_message(ToRendererMessage::CellStyle(styles.clone()));
        }
    }
}

fn get_cell_styles() -> Vec<CellStyle> {
    let mut styles = Vec::new();
    for _ in 0..200 {
        styles.push(CellStyle {
            text: String::from("AABB"),
            text_color: Color::BLACK,
            text_size: 40.0,
            text_alignment: common::communication::TextAlignment::Center,
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
            texture: None,
        });
    }
    styles
}
