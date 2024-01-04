use bevy::{
    app::{Plugin, Startup, Update},
    asset::Assets,
    ecs::{
        component::Component,
        entity::Entity,
        event::EventWriter,
        query::With,
        system::{Commands, EntityCommand, Query},
    },
    hierarchy::BuildWorldChildren,
    math::{vec2, vec3},
    prelude::SpatialBundle,
    render::{
        batching::NoAutomaticBatching,
        color::Color,
        mesh::Mesh,
        primitives::Aabb,
        view::{RenderLayers, Visibility},
    },
    window::{PrimaryWindow, Window},
};
use frontend::cell::{background::Background, create_mesh, CellMarker, CellMesh, CreateClipArea};
use rand::random;

use crate::cell_material::CellMaterial;

pub struct Testing;
impl Plugin for Testing {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, update_ball);
    }
}

fn setup(mut commands: Commands) {
    for _ in 0..100 {
        commands.spawn_empty().add(CreateClipArea).insert(Ball);
        //commands.spawn_empty().add(CreateCustomCell).insert(Ball);
    }
}

#[derive(Component)]
struct Ball;

fn update_ball(
    query: Query<Entity, With<Ball>>,
    mut set_style: EventWriter<frontend::cell::SetStyle>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window.single();

    for entity in query.iter() {
        set_style.send(frontend::cell::SetStyle {
            entity,
            style: frontend::cell::CellStyle {
                text: String::from("AABB"),
                text_color: Color::BLACK,
                text_size: 40.0,
                text_alignment: common::communication::TextAlignment::Center,
                text_position: vec2(0.0, 0.0),
                font: None,
                color: Color::rgb(random(), random(), random()),
                texture: None,
                pos: vec3(
                    (random::<f32>() - 0.5) * (window.physical_width() as f32 - 100.0) - 50.0,
                    (random::<f32>() - 0.5) * (window.physical_height() as f32 - 100.0) + 50.0,
                    0.0,
                ),
                size: vec2(random::<f32>() * 50.0 + 50.0, random::<f32>() * 50.0 + 50.0),
                skew: random::<f32>() * 50.0,
                visible: true,
                rounding: [
                    random::<f32>() * 50.0,
                    random::<f32>() * 50.0,
                    random::<f32>() * 50.0,
                    random::<f32>() * 50.0,
                ],
                render_layer: 0,
            },
        })
    }
}

pub struct CreateCustomCell;
impl EntityCommand for CreateCustomCell {
    fn apply(self, id: Entity, world: &mut bevy::prelude::World) {
        let background_id = create_background(world);

        world
            .entity_mut(id)
            .insert((
                SpatialBundle {
                    visibility: Visibility::Inherited,
                    ..Default::default()
                },
                RenderLayers::layer(0),
                Background(background_id),
                CellMarker,
            ))
            .add_child(background_id);
    }
}

fn create_background(world: &mut bevy::prelude::World) -> Entity {
    if !world.contains_resource::<CellMesh>() {
        let mut meshes = world.resource_mut::<Assets<Mesh>>();
        let mesh = meshes.add(create_mesh()).into();
        world.insert_resource(CellMesh { mesh });
    }
    let mesh = world.resource::<CellMesh>().mesh.clone();

    world
        .spawn((
            mesh,
            CellMaterial::default(),
            SpatialBundle::default(),
            Aabb::default(),
            RenderLayers::layer(0),
            NoAutomaticBatching,
        ))
        .id()
}
