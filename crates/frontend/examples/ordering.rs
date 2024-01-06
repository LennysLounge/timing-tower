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
) {
    commands.spawn(Camera2dBundle::default());

    for i in 0..5 {
        let i = i as f32;
        set_style.send(SetStyle {
            entity: commands.spawn_empty().add(CreateCell).id(),
            style: CellStyle {
                color: Color::hsl(i * 60.0, 1.0, 0.5),
                pos: vec3(-150.0 + 50.0 * i, 150.0 - 50.0 * i, i),
                size: vec2(200.0, 200.0),
                visible: true,
                ..Default::default()
            },
        });
    }

    set_style.send(SetStyle {
        entity: commands.spawn_empty().add(CreateCell).id(),
        style: CellStyle {
            color: Color::RED,
            pos: vec3(-150.0, 150.0, -1.0),
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
