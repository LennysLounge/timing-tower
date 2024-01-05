use bevy::{
    app::{Plugin, Startup, Update},
    ecs::{
        component::Component,
        entity::Entity,
        event::EventWriter,
        query::With,
        system::{Commands, EntityCommand, Query},
    },
    math::{vec2, vec3},
    prelude::SpatialBundle,
    render::{batching::NoAutomaticBatching, color::Color, view::RenderLayers},
    window::{PrimaryWindow, Window},
};
use frontend::{
    cell::{get_or_create_mesh, CellBundle, CellMarker},
    cell_material::CellMaterial,
};
use rand::random;

pub struct TestingPlugin;
impl Plugin for TestingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, update_ball);
    }
}

fn setup(mut commands: Commands) {
    for _ in 0..3200 {
        commands.spawn_empty().add(frontend::cell::CreateCell).insert(Ball);
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
        let mesh = get_or_create_mesh(world);
        world.entity_mut(id).insert(CellBundle {
            mesh,
            material: CellMaterial::default(),
            spatial_bundle: SpatialBundle::default(),
            render_layers: RenderLayers::layer(0),
            no_automatic_batching: NoAutomaticBatching,
            marker: CellMarker,
        });
    }
}
