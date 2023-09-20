use std::fs::{self};

use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    ecs::{system::EntityCommand, world::EntityMut},
    prelude::{
        App, AssetServer, Camera, Camera2dBundle, ClearColor, Color, Commands, Component,
        EventWriter, First, GlobalTransform, Handle, PreStartup, Query, Res, Resource, Startup,
        Transform, Vec2, Vec3, With, World,
    },
    text::Font,
    time::{Timer, TimerMode},
    DefaultPlugins,
};
use bevy_egui::EguiPlugin;
use cell::{init_cell, CellPlugin, CellStyle, SetStyle};
use editor::{EditorPlugin, EditorState};
use gradient_material::CustomMaterialPlugin;

use style_def::{Rounding, TimingTowerStyleDef};
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
        .add_systems(First, move_top_left)
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
    let font: Handle<Font> = asset_server.load("fonts/bahnschrift.ttf");
    commands.insert_resource(DefaultFont(font));
}

fn setup(mut commands: Commands, mut set_style_event: EventWriter<SetStyle>) {
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

    let background_id = commands
        .spawn_empty()
        .add(init_cell)
        .insert(BackgroundImage)
        .id();
    set_style_event.send(SetStyle {
        entity: background_id,
        style: CellStyle {
            text: "".to_string(),
            color: Color::WHITE,
            texture: Some("acc6.PNG".to_string()),
            pos: Vec3::new(0.0, 0.0, 0.0),
            size: Vec2::new(1920.0, 1080.0),
            skew: 0.0,
            visible: true,
            rounding: Rounding::default(),
        },
    });

    commands.insert_resource(EditorState {
        style_def: style_def.clone(),
    });

    commands
        .spawn_empty()
        .add(init_timing_tower(style_def, adapter));
}

fn move_top_left(
    main_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut bg_images: Query<&mut Transform, With<BackgroundImage>>,
) {
    let (camera, camera_transform) = main_camera.single();
    for mut bg_image in bg_images.iter_mut() {
        if let Some(top_left) = camera.viewport_to_world_2d(camera_transform, Vec2::new(0.0, 0.0)) {
            bg_image.translation = Vec3::new(top_left.x, top_left.y, -10.0);
        }
    }
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
