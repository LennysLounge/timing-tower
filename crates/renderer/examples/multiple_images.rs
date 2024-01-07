use bevy::math::vec2;
use bevy::prelude::*;

use bevy::{app::App, math::vec3};
use frontend::cell::{CellStyle, CreateCell, SetStyle};
use frontend::FrontendPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FrontendPlugin))
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.9)))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut set_style: EventWriter<SetStyle>,
    mut _meshes: ResMut<Assets<Mesh>>,
    mut _images: ResMut<Assets<Image>>,
    _asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());

    for i in 0..11 {
        let i = i as f32 / 10.0;
        set_style.send(SetStyle {
            entity: commands.spawn_empty().add(CreateCell).id(),
            style: CellStyle {
                color: Color::hsl(i * 360.0, 1.0, 0.50),
                pos: vec3(-500.0 + 500.0 * i, 150.0 - 300.0 * i, i),
                size: vec2(100.0, 100.0),
                visible: true,
                texture: Some(_asset_server.load("../../../savefile/constructors/Alpine.png")),
                ..Default::default()
            },
        });
    }

    for i in 0..11 {
        let i = i as f32 / 10.0;
        set_style.send(SetStyle {
            entity: commands.spawn_empty().add(CreateCell).id(),
            style: CellStyle {
                color: Color::hsl(i * 360.0, 1.0, 0.50),
                pos: vec3(-200.0 + 500.0 * i, 150.0 - 300.0 * i, 1.0 - i),
                size: vec2(100.0, 100.0),
                visible: true,
                texture: Some(_asset_server.load("../../../savefile/constructors/Porsche.png")),
                ..Default::default()
            },
        });
    }
}
