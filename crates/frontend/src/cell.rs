use bevy::{
    app::PostUpdate,
    ecs::{entity::Entity, schedule::IntoSystemConfigs, system::EntityCommand},
    math::Vec2,
    prelude::*,
    render::{
        batching::NoAutomaticBatching,
        texture::Image,
        view::{NoFrustumCulling, RenderLayers},
    },
    sprite::Anchor,
    text::{Font, Text, Text2dBundle, TextStyle},
};
use common::communication::TextAlignment;

use crate::cell_material::CellMaterial;

#[derive(Debug, PartialEq, Eq, Clone, Hash, SystemSet)]
pub struct CellSystem;

pub struct CellPlugin;
impl Plugin for CellPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<SetStyle>().add_systems(
            PostUpdate,
            (update_style, update_style_foreground)
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
#[derive(Default)]
pub struct CellStyle {
    pub text: String,
    pub text_color: Color,
    pub text_size: f32,
    pub text_alignment: TextAlignment,
    pub text_position: Vec2,
    pub font: Option<Handle<Font>>,
    pub color: Color,
    pub texture: Option<Handle<Image>>,
    pub pos: Vec3,
    pub size: Vec2,
    // Order top left, top right, bottom left, bottom right,
    pub corner_offsets: [Vec2; 4],
    pub visible: bool,
    // Order top left, top right, bottom left, bottom right,
    pub rounding: [f32; 4],
    pub render_layer: u8,
}

#[derive(Component)]
pub struct CellMarker;

#[derive(Bundle)]
pub struct CellBundle {
    pub material: CellMaterial,
    pub spatial_bundle: SpatialBundle,
    pub render_layers: RenderLayers,
    pub no_automatic_batching: NoAutomaticBatching,
    pub marker: CellMarker,
    pub no_furstrum_culling: NoFrustumCulling,
}

pub struct CreateCell;
impl EntityCommand for CreateCell {
    fn apply(self, id: Entity, world: &mut World) {
        let foreground_id = create_foreground(world);

        world
            .entity_mut(id)
            .insert((
                CellBundle {
                    material: CellMaterial::default(),
                    spatial_bundle: SpatialBundle::default(),
                    no_automatic_batching: NoAutomaticBatching,
                    render_layers: RenderLayers::layer(0),
                    marker: CellMarker,
                    no_furstrum_culling: NoFrustumCulling,
                },
                Foreground(foreground_id),
            ))
            .add_child(foreground_id);
    }
}

pub struct CreateClipArea;
impl EntityCommand for CreateClipArea {
    fn apply(self, id: Entity, world: &mut bevy::prelude::World) {
        world.entity_mut(id).insert(CellBundle {
            material: CellMaterial::default(),
            spatial_bundle: SpatialBundle::default(),
            no_automatic_batching: NoAutomaticBatching,
            render_layers: RenderLayers::layer(0),
            marker: CellMarker,
            no_furstrum_culling: NoFrustumCulling,
        });
    }
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

fn update_style(
    mut events: EventReader<SetStyle>,
    mut cells: Query<
        (
            &mut Transform,
            &mut Visibility,
            &mut RenderLayers,
            &mut CellMaterial,
        ),
        With<CellMarker>,
    >,
) {
    for SetStyle { entity, style } in events.read() {
        let Ok((mut transform, mut visibility, mut render_layers, mut material)) =
            cells.get_mut(*entity)
        else {
            println!("Cell not found for update");
            continue;
        };
        if style.visible {
            *visibility = Visibility::Inherited;
        } else {
            *visibility = Visibility::Hidden;
        }

        material.color = style.color;
        material.texture = style.texture.clone();
        material.size = style.size;
        material.corner_offsets = style.corner_offsets;
        material.rounding = style.rounding.into();

        transform.translation = style.pos;
        *render_layers = RenderLayers::layer(style.render_layer);
    }
}

#[derive(Component)]
pub struct Foreground(pub Entity);

pub fn update_style_foreground(
    cells: Query<&Foreground>,
    mut texts: Query<(&mut Text, &mut Anchor, &mut Transform, &mut RenderLayers)>,
    mut events: EventReader<SetStyle>,
) {
    for event in events.read() {
        let Ok(foreground) = cells.get(event.entity) else {
            continue;
        };

        let Ok((mut text, mut anchor, mut transform, mut render_layers)) =
            texts.get_mut(foreground.0)
        else {
            continue;
        };
        *text = Text::from_section(
            event.style.text.clone(),
            TextStyle {
                font: match event.style.font.as_ref() {
                    Some(handle) => handle.clone(),
                    None => Handle::<Font>::default(),
                },
                font_size: event.style.text_size,
                color: event.style.text_color,
            },
        );
        *anchor = match event.style.text_alignment {
            TextAlignment::Left => Anchor::CenterLeft,
            TextAlignment::Center => Anchor::Center,
            TextAlignment::Right => Anchor::CenterRight,
        };

        transform.translation = Vec3::new(
            event.style.text_position.x,
            -event.style.text_position.y,
            1.0,
        );

        *render_layers = RenderLayers::layer(event.style.render_layer);
    }
}
