use bevy::{
    prelude::{
        BuildChildren, Color, Commands, Component, Entity, EventReader, Plugin, PostUpdate, Query,
        Res, Transform, Update, Vec3, With,
    },
    text::{Text, Text2dBundle, TextStyle},
};

use crate::{cell::SetStyle, DefaultFont};

pub struct ForegroundPlugin;
impl Plugin for ForegroundPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, add_foreground)
            .add_systems(PostUpdate, update_style);
    }
}

#[derive(Component, Default)]
pub struct AddForeground;

#[derive(Component)]
pub struct Foreground(pub Entity);

fn add_foreground(
    mut commands: Commands,
    font: Res<DefaultFont>,
    entities: Query<Entity, With<AddForeground>>,
) {
    for entity in entities.iter() {
        let text = commands
            .spawn(Text2dBundle {
                text: Text::from_section(
                    "Text",
                    TextStyle {
                        font: font.0.clone(),
                        font_size: 100.0,
                        color: Color::WHITE,
                    },
                ),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                ..Default::default()
            })
            .id();

        let mut entity = commands.entity(entity);
        entity.remove::<AddForeground>();
        entity.add_child(text);
        entity.insert(Foreground(text));
    }
}

fn update_style(
    cells: Query<&Foreground>,
    mut texts: Query<(&mut Text, &mut Transform)>,
    mut events: EventReader<SetStyle>,
    font: Res<DefaultFont>,
) {
    for event in events.iter() {
        let Ok(foreground) = cells.get(event.entity) else {
            continue;
        };
        let Ok((mut text, mut transform)) = texts.get_mut(foreground.0) else {
            continue;
        };
        *text = Text::from_section(
            event.style.text.clone(),
            TextStyle {
                font: font.0.clone(),
                font_size: 20.0,
                color: Color::WHITE,
            },
        );
        transform.translation = Vec3::new(
            (event.style.size.x + event.style.skew) / 2.0,
            -event.style.size.y / 2.0,
            1.0,
        )
    }
}
