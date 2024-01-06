use bevy::math::vec2;
use bevy::prelude::*;

use bevy::{app::App, math::vec3};
use frontend::cell::{CellStyle, CreateCell, SetStyle};
use frontend::FrontendPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FrontendPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut set_style: EventWriter<SetStyle>,
    mut _meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());

    // for i in 0..11 {
    //     let i = i as f32;
    //     println!("{}", i / 10.0);
    //     set_style.send(SetStyle {
    //         entity: commands.spawn_empty().add(CreateCell).id(),
    //         style: CellStyle {
    //             color: Color::rgba(1.0, 0.0, 0.0, i / 10.0),
    //             pos: vec3(-500.0 + 100.0 * i, 150.0, 0.0),
    //             size: vec2(100.0, 100.0),
    //             visible: true,
    //             ..Default::default()
    //         },
    //     });
    // }

    set_style.send(SetStyle {
        entity: commands.spawn_empty().add(CreateCell).id(),
        style: CellStyle {
            color: Color::RED,
            pos: vec3(-500.0, 150.0, -1.0),
            size: vec2(300.0, 300.0),
            visible: true,
            ..Default::default()
        },
    });

    set_style.send(SetStyle {
        entity: commands.spawn_empty().add(CreateCell).id(),
        style: CellStyle {
            color: Color::rgba(0.0, 0.0, 0.0, 0.5),
            pos: vec3(-300.0, 150.0, -0.0),
            size: vec2(300.0, 300.0),
            visible: true,
            texture: Some(asset_server.load("Alpine.png")),
            ..Default::default()
        },
    });

    set_style.send(SetStyle {
        entity: commands.spawn_empty().add(CreateCell).id(),
        style: CellStyle {
            color: Color::BLUE,
            pos: vec3(-100.0, 150.0, -1.0),
            size: vec2(300.0, 300.0),
            visible: true,
            ..Default::default()
        },
    });

    // set_style.send(SetStyle {
    //     entity: commands.spawn_empty().add(CreateCell).id(),
    //     style: CellStyle {
    //         color: Color::BLUE,
    //         pos: vec3(-50.0, 50.0, -1.0),
    //         size: vec2(300.0, 300.0),
    //         visible: true,
    //         ..Default::default()
    //     },
    // });
}
