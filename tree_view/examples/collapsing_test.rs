use bevy::{
    prelude::{App, Commands, ResMut, Resource, Startup, Update},
    DefaultPlugins,
};
use bevy_egui::{
    egui::{self, collapsing_header::CollapsingState, Ui},
    EguiContexts, EguiPlugin,
};
use split_collapsing_state::SplitCollapsingState;

#[path = "../src/split_collapsing_state.rs"]
mod split_collapsing_state;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_systems(Update, egui)
        .add_systems(Startup, setup)
        .run();
}

#[derive(Resource)]
struct EditorState {}

fn setup(mut commands: Commands) {
    commands.insert_resource(EditorState {});
}

fn egui(mut ctx: EguiContexts, mut _state: ResMut<EditorState>) {
    egui::CentralPanel::default().show(ctx.ctx_mut(), |ui| {
        let id = ui.make_persistent_id("Collapsing");

        // let (button, header, body) = CollapsingState::load_with_default_open(ui.ctx(), id, true)
        //     .show_header(ui, |ui| {
        //         ui.label("This is a header");
        //     })
        //     .body(|ui| {
        //         //ui.label("This is the body");
        //     });

        let split_state = SplitCollapsingState::show_header(ui, id, |ui| {
            ui.label("This is the header");
        });

        ui.label("Other things");
        split_state.show_body(ui, |ui| {
            ui.label("this is the body");
        });

        ui.separator();

        let add_header = |ui: &mut Ui| {
            ui.label("This is a detached header");
        };

        let mut state = CollapsingState::load_with_default_open(ui.ctx(), id, true);
        let header_response = ui.horizontal(|ui| {
            let prev_item_spacing = ui.spacing_mut().item_spacing;
            ui.spacing_mut().item_spacing.x = 0.0; // the toggler button uses the full indent width
                                                   //let collapser = self.show_default_button_indented(ui);
            let collapser =
                state.show_toggle_button(ui, egui::collapsing_header::paint_default_icon);

            ui.spacing_mut().item_spacing = prev_item_spacing;
            (collapser, add_header(ui))
        });
        state.store(ui.ctx());

        // header.body(|ui| {
        //     ui.label("body");
        // });
        ui.label("hello");

        let mut state = CollapsingState::load_with_default_open(ui.ctx(), id, true);
        state.show_body_indented(&header_response.response, ui, |ui| {
            ui.label("detached body");
        })
    });
}
