use bevy::{
    app::{Plugin, Startup, Update},
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    ecs::{
        component::Component,
        query::With,
        system::{Query, Res, ResMut, Resource},
        world::World,
    },
    render::color::Color,
    text::{Text, TextSection, TextStyle},
    time::{Time, Timer, TimerMode},
    ui::{node_bundles::TextBundle, Style, Val},
};

pub struct FrameratePlugin;
impl Plugin for FrameratePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin)
            .init_resource::<FrameCounter>()
            .insert_resource(SecondTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .add_systems(Startup, setup)
            .add_systems(Update, text_update_system);
    }
}

#[derive(Resource)]
struct SecondTimer(Timer);

#[derive(Resource, Default)]
pub struct FrameCounter {
    count: i32,
    last_count: i32,
}
impl FrameCounter {
    pub fn inc(&mut self) {
        self.count += 1;
    }
}

fn setup(world: &mut World) {
    // Text with multiple sections
    world.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font_size: 60.0,
                    ..Default::default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 60.0,
                color: Color::GOLD,
                // If no font is specified, the default font (a minimal subset of FiraMono) will be used.
                ..Default::default()
            }),
            TextSection::new(
                " renders: ",
                TextStyle {
                    font_size: 60.0,
                    ..Default::default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 60.0,
                color: Color::GOLD,
                // If no font is specified, the default font (a minimal subset of FiraMono) will be used.
                ..Default::default()
            }),
        ])
        .with_style(Style {
            position_type: bevy::ui::PositionType::Absolute,
            left: Val::Px(5.0),
            top: Val::Px(5.0),
            ..Default::default()
        }),
        FpsText,
    ));
}

// A unit struct to help identify the FPS UI component, since there may be many Text components
#[derive(Component)]
struct FpsText;

fn text_update_system(
    diagnostics: Res<DiagnosticsStore>,
    time: Res<Time>,
    mut query: Query<&mut Text, With<FpsText>>,
    mut frame_counter: ResMut<FrameCounter>,
    mut second_timer: ResMut<SecondTimer>,
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                text.sections[1].value = format!("{value:>6.2}");
            }
        }
        text.sections[3].value = format!("{}", frame_counter.last_count);
    }

    second_timer.0.tick(time.delta());
    if second_timer.0.just_finished() {
        frame_counter.last_count = frame_counter.count;
        frame_counter.count = 0;
    }
}
