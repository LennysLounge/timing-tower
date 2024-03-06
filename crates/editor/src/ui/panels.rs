use backend::GameAdapterResource;
use bevy::{
    ecs::{
        query::With,
        system::{Query, Res, ResMut},
    },
    transform::components::Transform,
};
use bevy_egui::{
    egui::{self},
    EguiContexts,
};

use crate::{camera::EditorCamera, MainCamera};

use super::{UiMessage, UiMessages};

pub(super) fn top_panel(
    mut ctx: EguiContexts,
    mut messages: ResMut<UiMessages>,
    game_adapter: Res<GameAdapterResource>,
) {
    egui::TopBottomPanel::top("Top panel")
        .show_separator_line(false)
        .show(ctx.ctx_mut(), |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Save").clicked() {
                        messages.push(UiMessage::SaveStyleDefinition);
                        ui.close_menu();
                    }
                });
                let is_connected = game_adapter.adapter().is_some_and(|a| !a.is_finished());
                if is_connected {
                    if ui.button("Disconnect").clicked() {
                        messages.push(UiMessage::GameAdapterClose);
                        ui.close_menu();
                    }
                } else {
                    ui.menu_button("Connection", |ui| {
                        if ui.button("Connect Dummy").clicked() {
                            messages.push(UiMessage::GameAdapterConnectDummy);
                            ui.close_menu();
                        }
                        if ui.button("Connect ACC").clicked() {
                            messages.push(UiMessage::GameAdapterConnectACC);
                            ui.close_menu();
                        }
                    });
                }
                ui.menu_button("View", |ui| {
                    if ui.button("Reset camera").clicked() {
                        messages.push(UiMessage::CameraReset);
                        ui.close_menu();
                    }
                    if ui.button("Align camera").clicked() {
                        messages.push(UiMessage::CameraAlign);
                        ui.close_menu();
                    }
                });
                if ui.button("Undo").clicked() {
                    messages.push(UiMessage::Undo);
                }
                if ui.button("Redo").clicked() {
                    messages.push(UiMessage::Redo);
                }
            });
        });
}

pub(super) fn bottom_panel(
    mut ctx: EguiContexts,
    mut _messages: ResMut<UiMessages>,
    editor_camera: Query<(&EditorCamera, &Transform), With<MainCamera>>,
) {
    egui::TopBottomPanel::bottom("Bottom panel")
        .show_separator_line(false)
        .show(ctx.ctx_mut(), |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let (cam, trans) = editor_camera.single();
                let zoom = 1.0 / cam.scale * 100.0;
                ui.label(format!("Zoom: {:.0}%", zoom));
                ui.separator();
                ui.separator();
                ui.label(format!("Zoom exponent: {:.2}", cam.scale_exponent));
                ui.separator();
                ui.label(format!("pos: {:?}", trans.translation));
            });
        });
}
