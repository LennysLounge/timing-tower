use std::fs::{self};

use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    ecs::{system::EntityCommand, world::EntityMut},
    prelude::{
        App, AssetServer, Camera2dBundle, ClearColor, Color, Commands, Component, Handle,
        PreStartup, Res, Resource, Startup, World,
    },
    text::Font,
    time::{Timer, TimerMode},
    DefaultPlugins,
};
use bevy_egui::EguiPlugin;
use cell::CellPlugin;
use editor::{EditorPlugin, EditorState};
use gradient_material::CustomMaterialPlugin;

use style_def::TimingTowerStyleDef;
use timing_tower::{init_timing_tower, TimingTowerPlugin};
use unified_sim_model::Adapter;

mod cell;
mod editor;
mod gradient_material;
mod style_def;
mod timing_tower;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgba(0.1, 0.1, 0.1, 0.0)))
        .insert_resource(SimpleTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
        // Plugins
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(EguiPlugin)
        // Crate plugins
        .add_plugins(CellPlugin)
        .add_plugins(CustomMaterialPlugin)
        .add_plugins(TimingTowerPlugin)
        .add_plugins(EditorPlugin)
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

fn load(asset_server: Res<AssetServer>, mut commands: Commands) {
    let font: Handle<Font> = asset_server.load("fonts/FiraSans-bold.ttf");
    commands.insert_resource(DefaultFont(font));
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));

    let adapter = Adapter::new_dummy();
    commands.insert_resource(GameAdapterResource {
        adapter: adapter.clone(),
    });

    let s = match fs::read_to_string("style.json") {
        Err(e) => {
            eprintln!("Cannot read 'style.json': {}", e);
            return;
        }
        Ok(s) => s,
    };
    let style_def = match serde_json::from_str::<TimingTowerStyleDef>(&s) {
        Ok(o) => o,
        Err(e) => {
            println!("Error parsing json: {}", e);
            return;
        }
    };

    commands.insert_resource(EditorState {
        style_def: style_def.clone(),
    });

    commands
        .spawn_empty()
        .add(init_timing_tower(style_def, adapter));
}

// fn update(time: Res<Time>, mut timer: ResMut<SimpleTimer>, mut event: EventWriter<SetRowStyleDef>) {
//     timer.0.tick(time.delta());
//     if timer.0.just_finished() {
//         let Ok(style_str) = fs::read_to_string("style.json") else {
//             eprintln!("Cannot read style file");
//             return;
//         };

//         let style = match serde_json::from_str::<RowStyleDef>(&style_str) {
//             Ok(o) => o,
//             Err(e) => {
//                 eprintln!("Error parsing style file: {e}");
//                 return;
//             }
//         };

//         event.send(SetRowStyleDef { style });
//     }
// }

pub trait SpawnAndInitWorld {
    fn spawn_new<C: EntityCommand>(&mut self, command: C) -> EntityMut;
}

impl SpawnAndInitWorld for World {
    fn spawn_new<C: EntityCommand>(&mut self, command: C) -> EntityMut {
        let id = self.spawn_empty().id();
        command.apply(id, self);
        self.entity_mut(id)
    }
}
