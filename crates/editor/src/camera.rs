use backend::savefile::{Savefile, SavefileChanged};
use bevy::{
    app::{Plugin, Startup, Update},
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{
        component::Component,
        event::EventReader,
        query::With,
        schedule::IntoSystemConfigs,
        system::{Commands, Query, Res},
    },
    input::{
        mouse::{MouseButtonInput, MouseWheel},
        ButtonState,
    },
    math::{vec2, vec3, UVec2, Vec2},
    render::camera::{Camera, Viewport},
    transform::components::Transform,
    window::{PrimaryWindow, Window},
};
use bevy_egui::egui::Rect;

use crate::MainCamera;

pub const ZOOM_BASE: f32 = 0.9;

pub struct EditorCameraPlugin;
impl Plugin for EditorCameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (camera_drag, savefile_changed))
            .add_systems(Update, set_camera_viewport.after(crate::ui::UiSystem));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera, EditorCamera::new()));
}

#[derive(Component)]
pub struct EditorCamera {
    drag_position: Option<Vec2>,
    pub scale_exponent: f32,
    pub scale: f32,
    pub raw_viewport: Rect,
}
impl EditorCamera {
    fn new() -> Self {
        Self {
            drag_position: None,
            scale_exponent: 0.0,
            scale: 1.0,
            raw_viewport: Rect::NOTHING,
        }
    }
}

fn camera_drag(
    mut mouse_events: EventReader<MouseButtonInput>,
    mut scroll_events: EventReader<MouseWheel>,
    window: Query<&Window, With<PrimaryWindow>>,
    mut camera: Query<(&mut EditorCamera, &mut Transform, &Camera), With<MainCamera>>,
) {
    let window = window.single();

    let (mut camera_drag, mut camera_transform, camera) = camera.single_mut();

    let is_cursor_inside_viewport = window
        .cursor_position()
        .zip(camera.viewport.as_ref())
        .is_some_and(|(cursor_pos, viewport)| {
            let viewport_max =
                (viewport.physical_position + viewport.physical_size).as_vec2() - vec2(5.0, 5.0);
            let viewport_min = viewport.physical_position.as_vec2() + vec2(5.0, 5.0);

            viewport_max.x > cursor_pos.x
                && viewport_max.y > cursor_pos.y
                && viewport_min.x < cursor_pos.x
                && viewport_min.y < cursor_pos.y
        });
    if !is_cursor_inside_viewport {
        mouse_events.clear();
        scroll_events.clear();
        return;
    }

    for ev in mouse_events.read() {
        match ev.state {
            ButtonState::Pressed => {
                camera_drag.drag_position = window.cursor_position();
            }
            ButtonState::Released => {
                camera_drag.drag_position = None;
            }
        }
    }

    for ev in scroll_events.read() {
        camera_drag.scale_exponent += ev.y.signum();
        camera_drag.scale = ZOOM_BASE.powf(camera_drag.scale_exponent);
        camera_transform.scale = vec3(camera_drag.scale, camera_drag.scale, 1.0);
    }

    if let Some(drag_position) = camera_drag.drag_position {
        if let Some(cursor_position) = window.cursor_position() {
            let delta = drag_position - cursor_position;
            camera_transform.translation += vec3(delta.x, -delta.y, 0.0) * camera_drag.scale;
        }
        camera_drag.drag_position = window.cursor_position();
    }
}

// make camera only render to view not obstructed by UI
fn set_camera_viewport(
    primary_window: Query<&mut Window, With<PrimaryWindow>>,
    egui_settings: Res<bevy_egui::EguiSettings>,
    mut cameras: Query<(&mut Camera, &EditorCamera), With<MainCamera>>,
) {
    let (mut cam, state) = cameras.single_mut();

    let Ok(window) = primary_window.get_single() else {
        return;
    };

    let scale_factor = window.scale_factor() * egui_settings.scale_factor;

    let viewport_pos = state.raw_viewport.left_top().to_vec2() * scale_factor as f32;
    let viewport_size = state.raw_viewport.size() * scale_factor as f32;

    cam.viewport = Some(Viewport {
        physical_position: UVec2::new(viewport_pos.x as u32, viewport_pos.y as u32),
        // make the size always odd to prevent the camera from being between two pixels.
        physical_size: UVec2::new((viewport_size.x as u32) & !1, (viewport_size.y as u32) & !1),
        depth: 0.0..1.0,
    });
}

fn savefile_changed(
    savefile: Res<Savefile>,
    mut savefile_changed: EventReader<SavefileChanged>,
    mut camera: Query<(&mut EditorCamera, &mut Transform), With<MainCamera>>,
) {
    if !savefile_changed.read().any(|s| s.replace) {
        return;
    }

    let (mut camera_drag, mut camera_transform) = camera.single_mut();

    // Set the scale so that the entire scene is visible in the viewport.
    let horizontal_scale = savefile.style().scene.prefered_size.x
        / (camera_drag.raw_viewport.width() - 100.0).max(100.0);
    let vertical_scale = savefile.style().scene.prefered_size.y
        / (camera_drag.raw_viewport.height() - 100.0).max(100.0);

    let scale = vertical_scale.max(horizontal_scale);
    let exponent = scale.log(ZOOM_BASE);

    camera_drag.scale_exponent = exponent;
    camera_drag.scale = scale;
    camera_transform.scale = vec3(camera_drag.scale, camera_drag.scale, 1.0);

    camera_transform.translation.x = savefile.style().scene.prefered_size.x * 0.5;
    camera_transform.translation.y = savefile.style().scene.prefered_size.y * -0.5;
}
