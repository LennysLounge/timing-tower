use bevy::{
    app::PostUpdate,
    ecs::{
        schedule::{IntoSystemConfigs, SystemSet},
        system::Resource,
    },
    math::{vec2, vec3, Vec2, Vec4},
    prelude::{
        Assets, BuildWorldChildren, Color, Component, EntityWorldMut, EventReader, Handle, Mesh,
        Plugin, Query, SpatialBundle, Transform, Vec3, Visibility, With,
    },
    render::{mesh::Indices, primitives::Aabb, render_resource::PrimitiveTopology},
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    text::{Font, Text, Text2dBundle, TextStyle},
};

use crate::cell_material::{CellMaterial, Gradient};

use self::{background::Background, foreground::Foreground, style::SetStyle};

pub mod background;
pub mod foreground;
pub mod style;

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

#[derive(Component)]
pub struct CellMarker;

#[derive(Resource)]
struct CellMesh {
    mesh: Mesh2dHandle,
}

pub fn init_cell(mut entity: EntityWorldMut) {
    let (foreground_id, background_id) = entity.world_scope(|world| {
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
                rounding: Vec4::ZERO,
            });

        let background_id = world
            .spawn((
                MaterialMesh2dBundle {
                    mesh,
                    material,
                    ..Default::default()
                },
                Aabb::default(),
            ))
            .id();

        let foreground_id = world
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
            .id();
        (foreground_id, background_id)
    });

    entity
        .insert((
            SpatialBundle {
                visibility: Visibility::Inherited,
                ..Default::default()
            },
            Background(background_id),
            Foreground(foreground_id),
            CellMarker,
        ))
        .add_child(background_id)
        .add_child(foreground_id);
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
    mut cells: Query<(&mut Transform, &mut Visibility), With<CellMarker>>,
) {
    for SetStyle { entity, style } in events.read() {
        let Ok((mut transform, mut visibility)) = cells.get_mut(*entity) else {
            println!("Cell not found for update");
            continue;
        };
        if style.visible {
            *visibility = Visibility::Inherited;
        } else {
            *visibility = Visibility::Hidden;
        }
        transform.translation = style.pos;
    }
}
