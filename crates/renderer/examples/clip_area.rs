use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::math::vec2;
use bevy::prelude::*;

use bevy::render::camera::RenderTarget;
use bevy::render::render_resource::{
    Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
};
use bevy::render::view::RenderLayers;
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

    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size: Extent3d::default(),
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..Default::default()
    };
    image.resize(Extent3d {
        width: 300,
        height: 300,
        ..Default::default()
    });
    let image_handle = _images.add(image);

    commands
        .spawn(Camera2dBundle {
            camera: Camera {
                order: -1,
                target: RenderTarget::Image(image_handle.clone()),
                ..Default::default()
            },
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::rgba_u8(0, 0, 0, 0)),
            },
            ..Default::default()
        })
        .insert(RenderLayers::layer(1));
    
    /*
     * These are rendered to the clip area.
     */
    set_style.send(SetStyle {
        entity: commands.spawn_empty().add(CreateCell).id(),
        style: CellStyle {
            color: Color::BLUE,
            pos: vec3(-210.0, 50.0, 0.0),
            size: vec2(100.0, 100.0),
            visible: true,
            render_layer: 1,
            ..Default::default()
        },
    });

    set_style.send(SetStyle {
        entity: commands.spawn_empty().add(CreateCell).id(),
        style: CellStyle {
            color: Color::GREEN,
            pos: vec3(-50.0, 50.0, 0.0),
            size: vec2(100.0, 100.0),
            visible: true,
            render_layer: 1,
            ..Default::default()
        },
    });

    set_style.send(SetStyle {
        entity: commands.spawn_empty().add(CreateCell).id(),
        style: CellStyle {
            color: Color::PURPLE,
            pos: vec3(110.0, 50.0, 0.0),
            size: vec2(100.0, 100.0),
            visible: true,
            render_layer: 1,
            ..Default::default()
        },
    });

    /*
     * This is where its rendered to.
     */
    set_style.send(SetStyle {
        entity: commands.spawn_empty().add(CreateCell).id(),
        style: CellStyle {
            color: Color::WHITE,
            pos: vec3(-150.0, -50.0, 1.0),
            size: vec2(300.0, 300.0),
            visible: true,
            texture: Some(image_handle),
            rounding: [150.0, 0.0, 0.0, 0.0],
            ..Default::default()
        },
    });


    /*
    This is a free standing cell and should not be clipped
    */
    set_style.send(SetStyle {
        entity: commands.spawn_empty().add(CreateCell).id(),
        style: CellStyle {
            color: Color::YELLOW,
            pos: vec3(-50.0, 200.0, 0.0),
            size: vec2(100.0, 100.0),
            visible: true,
            render_layer: 0,
            ..Default::default()
        },
    });
}
