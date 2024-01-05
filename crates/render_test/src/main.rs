mod cell_material;
mod testing;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    math::vec2,
    prelude::*,
    render::{batching::NoAutomaticBatching, view::NoFrustumCulling},
    sprite::Mesh2dHandle,
    window::PresentMode,
};
use cell_material::{CellMaterial, CellMaterialPlugin};
use frontend::FrontendPlugin;
use testing::TestingPlugin;

fn main() {
    //env::set_var("RUST_BACKTRACE", "1");
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: PresentMode::Immediate,
                    ..Default::default()
                }),
                ..Default::default()
            }),
            CellMaterialPlugin,
            FrameTimeDiagnosticsPlugin,
            LogDiagnosticsPlugin::default(),
        ))
        .add_plugins(FrontendPlugin)
        .add_plugins(TestingPlugin)
        .add_systems(Startup, |mut c: Commands| {
            c.spawn(Camera2dBundle::default());
        })
        //.add_systems(Startup, _setup)
        .run();
}

fn _setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    let quad = Mesh2dHandle::from(meshes.add(shape::Quad::new(vec2(100.0, 100.0)).into()));
    let _image: Handle<Image> = asset_server.load("Porsche.png");

    for x in 6..=10 {
        for y in 1..=10 {
            let x = x as f32 / 10.0;
            let y = y as f32 / 10.0;
            commands.spawn((
                quad.clone(),
                SpatialBundle {
                    transform: Transform {
                        translation: Vec3::new(x * 500.0 - 250.0, y * 500.0 - 250.0, 0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                CellMaterial {
                    size: vec2(50.0, 50.0),
                    skew: 0.0,
                    rounding: [25.0 * y, 15.0 * y, 5.0 * y, 0.0],
                    color: Color::hsla(x * 360., y, 0.5, 1.0),
                    texture: None,
                },
                NoAutomaticBatching,
                NoFrustumCulling,
            ));
        }
    }
    for x in 1..=5 {
        for y in 1..=10 {
            let x = x as f32 / 10.0;
            let y = y as f32 / 10.0;
            commands.spawn((
                quad.clone(),
                SpatialBundle {
                    transform: Transform {
                        translation: Vec3::new(x * 500.0 - 250.0, y * 500.0 - 250.0, 0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                CellMaterial {
                    size: vec2(50.0, 50.0) * x * 2.0 * y,
                    skew: -10.0 * y,
                    rounding: [0.0, 0.0, 0.0, 0.0],
                    color: Color::hsla(x * 360., y, 0.5, 1.0),
                    texture: None,
                },
                NoAutomaticBatching,
                NoFrustumCulling,
            ));
        }
    }
}
