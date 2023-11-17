use bevy::{
    app::PostUpdate,
    ecs::schedule::{IntoSystemConfigs, SystemSet},
    math::{Vec2, Vec4},
    prelude::{
        shape, Assets, BuildWorldChildren, Color, Component, EntityWorldMut, EventReader, Handle,
        Mesh, Plugin, Query, SpatialBundle, Transform, Vec3, Visibility, With,
    },
    render::primitives::Aabb,
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

pub fn init_cell(mut entity: EntityWorldMut) {
    let (foreground_id, background_id) = entity.world_scope(|world| {
        let mesh: Mesh2dHandle = world
            .resource_mut::<Assets<Mesh>>()
            .add(shape::RegularPolygon::new(50.0, 80).into())
            .into();

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
