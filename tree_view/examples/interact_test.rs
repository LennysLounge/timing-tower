use bevy::{
    prelude::{App, Commands, ResMut, Resource, Startup, Update},
    DefaultPlugins,
};
use bevy_egui::{
    egui::{self, pos2, vec2, Color32, Rect, Rounding, Sense},
    EguiContexts, EguiPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_systems(Update, egui)
        .add_systems(Startup, setup)
        .run();
}

#[derive(Resource)]
struct EditorState;

fn setup(mut commands: Commands, mut _ctx: EguiContexts) {
    commands.insert_resource(EditorState);

    //dear_egui::set_theme(ctx.ctx_mut(), dear_egui::SKY);
}

fn egui(mut ctx: EguiContexts, mut _state: ResMut<EditorState>) {
    egui::CentralPanel::default().show(ctx.ctx_mut(), |ui| {
        let rect = Rect::from_min_size(pos2(10.0, 10.0), vec2(200.0, 200.0));

        ui.painter()
            .rect_filled(rect, Rounding::none(), Color32::BLUE);

        let id = ui.id().with("background interact");
        let background_res = ui.interact(rect, id, Sense::click());
        if background_res.clicked() {
            println!("Background clicked");
        }

        let button_res = ui.button("Button");
        if button_res.clicked() {
            println!("button clicked");
        }

        let combiend = background_res.union(button_res);
        if combiend.clicked(){
            println!(" combined clicked");
        }

    });
}
