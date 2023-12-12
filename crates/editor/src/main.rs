use asset_path_store::EditorAssetPathStorePlugin;
use backend::{
    savefile::{Savefile, SavefileChanged},
    BackendPlugin,
};
use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    ecs::system::{EntityCommand, ResMut},
    prelude::{
        App, AssetServer, ClearColor, Color, Commands, Component, EntityWorldMut, EventWriter,
        Handle, PreStartup, Res, Resource, Startup, Vec2, Vec3, World,
    },
    text::Font,
    time::{Timer, TimerMode},
    DefaultPlugins,
};
use bevy_egui::EguiPlugin;
use editor::EditorPlugin;
use frontend::{
    cell::{
        init_cell,
        style::{CellStyle, SetStyle, TextAlignment},
    },
    FrontendPlugin,
};
use std::env;
use uuid::uuid;

use timing_tower::{init_timing_tower, TimingTowerPlugin};
use unified_sim_model::Adapter;

mod asset_path_store;
mod editor;
mod properties;
mod reference_store;
mod style;
mod timing_tower;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    App::new()
        .insert_resource(ClearColor(Color::rgba(0.1, 0.1, 0.1, 0.0)))
        .insert_resource(SimpleTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .add_plugins(BackendPlugin)
        .add_plugins(EditorAssetPathStorePlugin)
        .add_plugins(TimingTowerPlugin)
        .add_plugins(EditorPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(EguiPlugin)
        .add_plugins(FrontendPlugin)
        // Systems
        .add_systems(PreStartup, load)
        .add_systems(Startup, setup)
        .run();
}

#[derive(Resource)]
struct SimpleTimer(Timer);

#[derive(Resource)]
pub struct DefaultFont(pub Handle<Font>);

#[derive(Resource)]
#[allow(unused)]
pub struct GameAdapterResource {
    adapter: Adapter,
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
struct BackgroundImage;

fn load(asset_server: Res<AssetServer>, mut commands: Commands) {
    //let font: Handle<Font> = asset_server.load("fonts/FiraSans-bold.ttf");
    //let font: Handle<Font> = asset_server.load("fonts/Heebo-Regular.ttf");
    //let font: Handle<Font> = asset_server.load("fonts/Heebo-Black.ttf");
    let font: Handle<Font> = asset_server.load("fonts/cufonfonts D-DIN-Bold.otf");
    commands.insert_resource(DefaultFont(font));
}

fn setup(
    savefile_changed_event: EventWriter<SavefileChanged>,
    mut commands: Commands,
    mut set_style_event: EventWriter<SetStyle>,
    mut savefile: ResMut<Savefile>,
) {
    let adapter = Adapter::new_dummy();
    commands.insert_resource(GameAdapterResource {
        adapter: adapter.clone(),
    });

    savefile.load("savefile/style.style.json", savefile_changed_event);

    let background_id = commands
        .spawn_empty()
        .add(init_cell)
        .insert(BackgroundImage)
        .id();
    set_style_event.send(SetStyle {
        entity: background_id,
        style: CellStyle {
            text: "".to_string(),
            text_color: Color::BLACK,
            text_size: 20.0,
            text_alignment: TextAlignment::Center,
            text_position: Vec2::ZERO,
            color: Color::WHITE,
            texture: Some(uuid!("819d2f30-0d03-413a-8f09-9a0afa58b3ed")),
            pos: Vec3::new(0.0, 0.0, 0.0),
            size: Vec2::new(1920.0, 1080.0),
            skew: 0.0,
            visible: true,
            rounding: [0.0, 0.0, 0.0, 0.0],
        },
    });
    commands.spawn_empty().add(init_timing_tower(adapter));
}

pub trait SpawnAndInitWorld {
    fn spawn_new<C: EntityCommand>(&mut self, command: C) -> EntityWorldMut;
}

impl SpawnAndInitWorld for World {
    fn spawn_new<C: EntityCommand>(&mut self, command: C) -> EntityWorldMut {
        let id = self.spawn_empty().id();
        command.apply(id, self);
        self.entity_mut(id)
    }
}
