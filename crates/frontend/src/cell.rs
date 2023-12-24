use bevy::{
    app::PostUpdate,
    ecs::{
        entity::Entity,
        event::Event,
        schedule::{IntoSystemConfigs, SystemSet},
        system::{EntityCommand, Resource},
    },
    math::{vec2, vec3, Vec2, Vec4},
    prelude::{
        Assets, BuildWorldChildren, Color, Component, EventReader, Handle, Mesh, Plugin, Query,
        SpatialBundle, Transform, Vec3, Visibility, With,
    },
    render::{
        mesh::Indices, primitives::Aabb, render_resource::PrimitiveTopology, view::RenderLayers,
    },
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    text::{Font, Text, Text2dBundle, TextStyle},
};

use common::communication::CellStyle;

use crate::cell_material::{CellMaterial, Gradient};

use self::{background::Background, foreground::Foreground};

pub mod background;
pub mod foreground;

#[derive(Debug, PartialEq, Eq, Clone, Hash, SystemSet)]
pub struct CellSystem;

pub struct CellPlugin;
impl Plugin for CellPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<SetStyle>().add_systems(
            PostUpdate,
            (
                update_style,
                foreground::update_style,
                background::update_style,
            )
                .in_set(CellSystem)
                .before(bevy::text::update_text2d_layout),
        );
    }
}

#[derive(Event)]
pub struct SetStyle {
    pub entity: Entity,
    pub style: CellStyle,
}

#[derive(Component)]
pub struct CellMarker;

#[derive(Resource)]
struct CellMesh {
    mesh: Mesh2dHandle,
}

pub struct CreateCell;
impl EntityCommand for CreateCell {
    fn apply(self, id: Entity, world: &mut bevy::prelude::World) {
        let background_id = create_background(world);
        let foreground_id = create_foreground(world);

        world
            .entity_mut(id)
            .insert((
                SpatialBundle {
                    visibility: Visibility::Inherited,
                    ..Default::default()
                },
                RenderLayers::layer(0),
                Background(background_id),
                Foreground(foreground_id),
                CellMarker,
            ))
            .add_child(background_id)
            .add_child(foreground_id);
    }
}

pub struct CreateClipArea;
impl EntityCommand for CreateClipArea {
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

    let material = world
        .resource_mut::<Assets<CellMaterial>>()
        .add(CellMaterial {
            color: Color::PURPLE,
            gradient: Gradient::None,
            texture: None,
            size: Vec2::ZERO,
            skew: 0.0,
            rounding: Vec4::ZERO,
        });

    world
        .spawn((
            MaterialMesh2dBundle {
                mesh,
                material,
                ..Default::default()
            },
            Aabb::default(),
            RenderLayers::layer(0),
        ))
        .id()
}

fn create_foreground(world: &mut bevy::prelude::World) -> Entity {
    world
        .spawn(Text2dBundle {
            text: Text::from_section(
                "World",
                TextStyle {
                    font: Handle::<Font>::default(),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
            ..Default::default()
        })
        .insert(RenderLayers::layer(0))
        .id()
}

fn create_mesh() -> Mesh {
    let positions = vec![
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 0.0, 0.0),
    ];
    let normals = vec![
        vec3(0.0, 0.0, 1.0),
        vec3(0.0, 0.0, 1.0),
        vec3(0.0, 0.0, 1.0),
        vec3(0.0, 0.0, 1.0),
    ];
    let uvs = vec![
        vec2(0.0, 0.0),
        vec2(1.0, 0.0),
        vec2(1.0, 1.0),
        vec2(0.0, 1.0),
    ];
    let indices = vec![0, 1, 2, 0, 2, 3];

    Mesh::new(PrimitiveTopology::TriangleList)
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
        .with_indices(Some(Indices::U32(indices)))
}

fn update_style(
    mut events: EventReader<SetStyle>,
    mut cells: Query<(&mut Transform, &mut Visibility, &mut RenderLayers), With<CellMarker>>,
) {
    for SetStyle { entity, style } in events.read() {
        let Ok((mut transform, mut visibility, mut render_layers)) = cells.get_mut(*entity) else {
            println!("Cell not found for update");
            continue;
        };
        if style.visible {
            *visibility = Visibility::Inherited;
        } else {
            *visibility = Visibility::Hidden;
        }
        transform.translation = style.pos;
        *render_layers = RenderLayers::layer(style.render_layer);
    }
}
