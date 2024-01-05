use bevy::{
    app::{Plugin, PostUpdate, Startup, Update},
    asset::Assets,
    ecs::{
        component::Component,
        entity::Entity,
        event::{EventReader, EventWriter},
        query::With,
        schedule::IntoSystemConfigs,
        system::{Commands, EntityCommand, Local, Query},
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
use frontend::cell::{
    background::Background, create_mesh, CellMarker, CellMesh, CellStyle, CellSystem, SetStyle,
};
use rand::random;

use crate::cell_material::CellMaterial;

pub struct TestingPlugin;
impl Plugin for TestingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, update_ball)
            .add_systems(PostUpdate, update_style.in_set(CellSystem));
    }
}

fn setup(mut commands: Commands) {
    for _ in 0..200 {
        //commands.spawn_empty().add(frontend::cell::CreateClipArea).insert(Ball);
        commands.spawn_empty().add(CreateCustomCell).insert(Ball);
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

pub fn update_style(
    mut events: EventReader<SetStyle>,
    cells: Query<&Background>,
    mut background: Query<(&mut CellMaterial, &mut Aabb, &mut RenderLayers)>,
) {
    for event in events.read() {
        let Ok(background_hadle) = cells.get(event.entity) else {
            continue;
        };
        let Ok((mut material, mut aabb, mut render_layers)) =
            background.get_mut(background_hadle.0)
        else {
            continue;
        };

        material.color = event.style.color;
        material.texture = event.style.texture.clone();
        material.size = event.style.size;
        material.skew = event.style.skew;
        material.rounding = event.style.rounding.into();

        let (min_skew, max_skew) = if event.style.skew > 0.0 {
            (0.0, event.style.skew)
        } else {
            (event.style.skew, 0.0)
        };
        *aabb = Aabb::from_min_max(
            vec3(min_skew, -event.style.size.y, 0.0),
            vec3(event.style.size.x + max_skew, 0.0, 0.0),
        );

        *render_layers = RenderLayers::layer(event.style.render_layer);
    }
}
