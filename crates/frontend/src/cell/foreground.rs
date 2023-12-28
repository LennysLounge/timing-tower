use bevy::{
    prelude::{Component, Entity, EventReader, Handle, Query, Transform, Vec3},
    render::view::RenderLayers,
    sprite::Anchor,
    text::{Font, Text, TextStyle},
};
use common::communication::TextAlignment;

use crate::cell::SetStyle;

#[derive(Component)]
pub struct Foreground(pub Entity);

pub fn update_style(
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
