use backend::{
    savefile::{Savefile, SavefileChanged},
    BackendPlugin,
};
use ball::Ball;
use bevy::{
    app::Startup,
    ecs::{event::EventWriter, system::Commands},
    prelude::{App, ResMut, Resource},
    time::{Timer, TimerMode},
    transform::components::Transform,
    DefaultPlugins,
};

use webserver::WebserverPlugin;
use websocket::WebsocketPlugin;

use crate::ball::BallPlugin;

mod ball;
mod webserver;
mod websocket;

fn main() {
    App::new()
        .add_plugins(BackendPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugins(WebsocketPlugin)
        .add_plugins(WebserverPlugin)
        .add_plugins(BallPlugin)
        .insert_resource(RenderTimer(Timer::from_seconds(
            0.001,
            TimerMode::Repeating,
        )))
        .add_systems(Startup, (load_savefile, spawn_balls))
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

fn spawn_balls(mut commands: Commands) {
    for _ in 0..200 {
        commands.spawn((Transform::default(), Ball::new()));
    }
}
