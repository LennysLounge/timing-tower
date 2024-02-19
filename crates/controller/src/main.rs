use backend::{
    savefile::{Savefile, SavefileChanged},
    style_batcher::{PrepareBatcher, StyleBatcher},
    BackendPlugin, GameAdapterResource,
};
use bevy::{
    app::{PostUpdate, Startup},
    ecs::{
        event::EventWriter,
        schedule::IntoSystemConfigs,
        system::{Commands, Query},
    },
    prelude::{App, ResMut, Resource},
    time::{Timer, TimerMode},
    DefaultPlugins,
};

use common::communication::ToRendererMessage;
use ui::UiPlugin;
use unified_sim_model::Adapter;
use webserver::WebserverPlugin;
use websocket::{ClientState, WebsocketClient, WebsocketPlugin};

mod ball;
mod ui;
mod webserver;
mod websocket;

fn main() {
    App::new()
        .add_plugins(BackendPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_egui::EguiPlugin)
        .add_plugins(WebsocketPlugin)
        .add_plugins(WebserverPlugin)
        //.add_plugins(crate::ball::BallPlugin)
        .add_plugins(UiPlugin)
        .insert_resource(RenderTimer(Timer::from_seconds(
            0.001,
            TimerMode::Repeating,
        )))
        .add_systems(Startup, setup)
        .add_systems(PostUpdate, send_style_commands.after(PrepareBatcher))
        .run();
}

#[derive(Resource)]
struct RenderTimer(Timer);

fn setup(
    mut commands: Commands,
    mut savefile: ResMut<Savefile>,
    savefile_changed_event: EventWriter<SavefileChanged>,
) {
    savefile.load("../../savefile/style.json", savefile_changed_event);

    commands.insert_resource(GameAdapterResource {
        adapter: Adapter::new_acc(),
    });
}

fn send_style_commands(
    mut style_batcher: ResMut<StyleBatcher>,
    mut connections: Query<&mut WebsocketClient>,
) {
    let style_commands = style_batcher.drain();
    for mut client in connections.iter_mut() {
        if client.state() == &ClientState::Ready {
            client.send_message(ToRendererMessage::Style(style_commands.clone()));
        }
    }
}
