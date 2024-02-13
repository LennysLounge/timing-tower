use backend::{graphic::GraphicStates, savefile::Savefile, GameAdapterResource};
use bevy::{
    app::{Plugin, Startup, Update},
    ecs::{
        schedule::{IntoSystemConfigs, SystemSet},
        system::{Res, ResMut},
    },
};
use bevy_egui::{egui, EguiContexts};

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup_egui_context)
            .add_systems(Update, ui.in_set(UiSystem));
    }
}

#[derive(SystemSet, Hash, Debug, PartialEq, Eq, Clone)]
pub struct UiSystem;

fn setup_egui_context(mut ctx: EguiContexts) {
    //egui_extras::install_image_loaders(ctx.ctx_mut());
    dear_egui::set_theme(
        ctx.ctx_mut(),
        dear_egui::Theme::Sky,
        dear_egui::Font::OpenSans,
    );
}

fn ui(
    mut ctx: EguiContexts,
    adapter: Res<GameAdapterResource>,
    savefile: Res<Savefile>,
    mut graphic_states: ResMut<GraphicStates>,
) {
    egui::SidePanel::left("Side panel").show(ctx.ctx_mut(), |ui| {
        backend::ui::dashboard::show_entry_table(ui, &adapter.adapter);
        ui.allocate_space(ui.available_size_before_wrap());
    });
    egui::CentralPanel::default().show(ctx.ctx_mut(), |ui| {
        for graphic in savefile.style().graphics.contained_graphics() {
            backend::ui::dashboard::show_graphic(ui, graphic, &mut *graphic_states);
        }
    });
}
