use bevy::{
    prelude::{
        shape, Assets, BuildWorldChildren, Bundle, Color, Component, Entity, Event, EventReader,
        Mesh, Plugin, PostUpdate, Query, SpatialBundle, Transform, Vec2, Vec3, Visibility, With,
        World,
    },
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    text::{Text, Text2dBundle, TextStyle},
};

use crate::{
    gradient_material::{Gradient, GradientMaterial},
    style_def::Rounding,
    DefaultFont,
};

use self::{
    background::{AddBackground, Background, BackgroundPlugin},
    foreground::{AddForeground, Foreground, ForegroundPlugin},
};

pub mod background;
pub mod foreground;

pub struct CellPlugin;
impl Plugin for CellPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(BackgroundPlugin)
            .add_plugins(ForegroundPlugin)
            .add_event::<SetStyle>()
            .add_systems(PostUpdate, update_style);
    }
}
pub struct CellStyle {
    pub text: String,
    pub color: Color,
    pub pos: Vec3,
    pub size: Vec2,
    pub skew: f32,
    pub visible: bool,
    pub rounding: Rounding,
}

#[derive(Event)]
pub struct SetStyle {
    pub entity: Entity,
    pub style: CellStyle,
}

#[derive(Component)]
pub struct CellMarker;

#[derive(Bundle, Default)]
pub struct CellBundle {
    pub spatial: SpatialBundle,
    pub add_background: AddBackground,
    pub add_foreground: AddForeground,
}

pub fn init_cell(entity_id: Entity, world: &mut World) {
    let mesh: Mesh2dHandle = world
        .resource_mut::<Assets<Mesh>>()
        .add(shape::RegularPolygon::new(50.0, 80).into())
        .into();

    let material = world
        .resource_mut::<Assets<GradientMaterial>>()
        .add(GradientMaterial {
            color: Color::PURPLE,
            gradient: Gradient::None,
            texture: None,
        });

    let background_id = world
        .spawn(MaterialMesh2dBundle {
            mesh,
            material,
            ..Default::default()
        })
        .id();

    let default_font = world.resource::<DefaultFont>().0.clone();
    let foreground_id = world
        .spawn(Text2dBundle {
            text: Text::from_section(
                "World",
                TextStyle {
                    font: default_font,
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
            ..Default::default()
        })
        .id();

    world
        .entity_mut(entity_id)
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

pub fn update_style(
    mut events: EventReader<SetStyle>,
    mut cells: Query<(&mut Transform, &mut Visibility), With<CellMarker>>,
) {
    for SetStyle { entity, style } in events.iter() {
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
