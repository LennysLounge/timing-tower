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

    set_style.send(SetStyle {
        entity: commands.spawn_empty().add(CreateCell).id(),
        style: CellStyle {
            color: Color::WHITE,
            pos: vec3(0.0, 150.0, 1.0),
            size: vec2(300.0, 300.0),
            visible: true,
            texture: Some(_asset_server.load("../../../savefile/constructors/Porsche.png")),
            rounding: [0.0, 0.0, 0.0, 0.0],
            ..Default::default()
        },
    });
    set_style.send(SetStyle {
        entity: commands.spawn_empty().add(CreateCell).id(),
        style: CellStyle {
            color: Color::WHITE,
            pos: vec3(-300.0, 150.0, -1.0),
            size: vec2(300.0, 300.0),
            visible: true,
            texture: Some(_asset_server.load("../../../savefile/constructors/Alpine.png")),
            rounding: [0.0, 0.0, 0.0, 0.0],
            ..Default::default()
        },
    });
    // set_style.send(SetStyle {
    //     entity: commands.spawn_empty().add(CreateCell).id(),
    //     style: CellStyle {
    //         color: Color::WHITE,
    //         pos: vec3(300.0, 150.0, 1.0),
    //         size: vec2(300.0, 300.0),
    //         visible: true,
    //         texture: Some(_asset_server.load("../../../savefile/constructors/BMW.png")),
    //         rounding: [0.0, 0.0, 0.0, 0.0],
    //         ..Default::default()
    //     },
    // });
}
