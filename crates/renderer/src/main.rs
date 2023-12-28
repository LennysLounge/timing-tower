mod asset_path_store;
mod cell_manager;
mod framerate;
mod websocket;

use asset_path_store::WebAssetPathStorePlugin;
use bevy::{
    app::{PluginGroup, Update},
    asset::AssetMetaCheck,
    ecs::{
        component::Component,
        event::EventWriter,
        query::With,
        system::{Commands, Query, Res, ResMut, Resource},
    },
    gizmos::gizmos::Gizmos,
    input::mouse::MouseButtonInput,
    math::{vec2, vec3, Vec2},
    prelude::{App, Camera2dBundle, ClearColor, Color, EventReader, Startup},
    transform::components::Transform,
    window::{PrimaryWindow, Window, WindowPlugin, WindowResized},
    DefaultPlugins,
};
use cell_manager::CellManagerPlugin;
use common::communication::{ToControllerMessage, ToRendererMessage};
use framerate::FrameratePlugin;
use frontend::{
    cell::{CreateCell, SetStyle},
    FrontendPlugin,
};

use websocket::{ReceivedMessage, SendMessage, WebsocketPlugin};

fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.9)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                fit_canvas_to_parent: true,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(FrontendPlugin)
        .add_plugins((
            WebsocketPlugin,
            FrameratePlugin,
            WebAssetPathStorePlugin,
            CellManagerPlugin,
        ))
        .add_systems(Startup, setup_camera)
        //.add_systems(Startup, setup_cell)
        .init_resource::<SceneDefinition>()
        .add_systems(
            Update,
            (mouse_click_send_message, init_scene, gizmos, update_camera),
        )
        .run();
}

#[allow(unused)]
fn setup_cell(mut commands: Commands, mut set_style: EventWriter<SetStyle>) {
    let cell_id = commands.spawn_empty().add(CreateCell).id();
    set_style.send(SetStyle {
        entity: cell_id,
        style: frontend::cell::CellStyle {
            text: String::from(""),
            text_color: Color::BLACK,
            text_size: 40.0,
            text_alignment: common::communication::TextAlignment::Center,
            text_position: vec2(0.0, 0.0),
            color: Color::BLUE,
            texture: None,
            pos: vec3(320.0, 540.0, 0.0),
            size: vec2(640.0, 360.0),
            skew: 0.0,
            visible: true,
            rounding: [100.0, 200.0, 50.0, 150.0],
            render_layer: 0,
        },
    });
}

#[derive(Component)]
struct MainCamera;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default()).insert(MainCamera);
}

fn mouse_click_send_message(
    mut mouse_event: EventReader<MouseButtonInput>,
    mut send_message: EventWriter<SendMessage>,
) {
    for ev in mouse_event.read() {
        if let bevy::input::ButtonState::Pressed = ev.state {
            send_message.send(SendMessage {
                message: ToControllerMessage::Debug("Mouse pressed".to_owned()),
            });
        }
    }
}

#[derive(Resource, Default)]
struct SceneDefinition {
    prefered_size: Vec2,
}
fn init_scene(
    mut scene: ResMut<SceneDefinition>,
    mut received_messages: EventReader<ReceivedMessage>,
    mut cameras: Query<&mut Transform, With<MainCamera>>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let Some(prefered_size) = received_messages
        .read()
        .filter_map(|x| match x.message {
            ToRendererMessage::Init { prefered_size, .. } => Some(prefered_size),
            _ => None,
        })
        .last()
    else {
        return;
    };
    scene.prefered_size = prefered_size;

    for mut camera in cameras.iter_mut() {
        let window = window.single();
        let x = window.physical_width() as f32 * 0.5;
        let y = window.physical_height() as f32 * -0.5;
        camera.translation = vec3(x, y, 0.0);
    }
}

fn gizmos(mut gizmos: Gizmos, scene: Res<SceneDefinition>) {
    gizmos.rect_2d(
        scene.prefered_size * vec2(0.5, -0.5),
        0.0,
        scene.prefered_size,
        Color::BLUE,
    );
}

fn update_camera(
    mut resized: EventReader<WindowResized>,
    mut cameras: Query<&mut Transform, With<MainCamera>>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    if resized.is_empty() {
        return;
    }
    resized.clear();

    for mut camera in cameras.iter_mut() {
        let window = window.single();
        let x = window.physical_width() as f32 * 0.5;
        let y = window.physical_height() as f32 * -0.5;
        camera.translation = vec3(x, y, 0.0);
    }
}
