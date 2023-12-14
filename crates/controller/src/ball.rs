use bevy::{
    app::{Plugin, Update},
    ecs::{component::Component, system::Query},
    math::{vec2, Vec2, Vec3},
    render::color::Color,
    transform::components::Transform,
};
use common::communication::{CellStyle, StyleCommand, TextAlignment, ToRendererMessage};
use uuid::Uuid;

use crate::websocket::{ClientState, WebsocketClient};

pub struct BallPlugin;
impl Plugin for BallPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, update);
    }
}

#[derive(Component)]
pub struct Ball {
    id: Uuid,
    velocity: Vec3,
    color: Color,
}

impl Ball {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            velocity: Vec3::new(
                rand::random::<f32>() * 5.0,
                rand::random::<f32>() * 5.0,
                0.0,
            ),
            color: Color::Hsla {
                hue: rand::random::<f32>() * 360.0,
                saturation: rand::random::<f32>(),
                lightness: rand::random::<f32>(),
                alpha: 1.0,
            },
        }
    }
}

fn update(
    mut balls: Query<(&mut Ball, &mut Transform)>,
    mut connections: Query<&mut WebsocketClient>,
) {
    let mut styles = Vec::new();

    for (mut ball, mut transform) in balls.iter_mut() {
        transform.translation += ball.velocity;

        if transform.translation.x <= 0.0 {
            transform.translation.x *= -1.0;
            ball.velocity.x *= -1.0;
        }
        if transform.translation.y <= 0.0 {
            transform.translation.y *= -1.0;
            ball.velocity.y *= -1.0;
        }
        if transform.translation.x >= 1280.0 {
            transform.translation.x -= (transform.translation.x - 1280.0) * 2.0;
            ball.velocity.x *= -1.0;
        }
        if transform.translation.y >= 720.0 {
            transform.translation.y -= (transform.translation.y - 720.0) * 2.0;
            ball.velocity.y *= -1.0;
        }

        styles.push(StyleCommand {
            id: ball.id,
            style: CellStyle {
                text: String::from("AABB"),
                text_color: Color::BLACK,
                text_size: 40.0,
                text_alignment: TextAlignment::Center,
                text_position: Vec2::ZERO,
                color: ball.color.clone(),
                texture: None,
                pos: transform.translation,
                size: vec2(30.0, 30.0),
                skew: 0.0,
                visible: true,
                rounding: [15.0; 4],
            },
        })
    }

    for mut client in connections.iter_mut() {
        if client.state() == &ClientState::Ready {
            client.send_message(ToRendererMessage::Style(styles.clone()));
        }
    }
}
