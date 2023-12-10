use bevy::{
    app::PluginGroup,
    prelude::{App, Startup, Update},
    window::{Window, WindowPlugin, WindowResolution},
    DefaultPlugins,
};
use bevy_egui::{
    egui::{self},
    EguiContexts, EguiPlugin,
};
use tree_view::v2::TreeViewBuilder;

mod data;
use data::*;
use visitor::{
    DropAllowedVisitor, InsertNodeVisitor, PrintTreeListing, RemoveNodeVisitor, TreeViewVisitor,
};

mod visitor;

fn main() {
    let mut tree = make_tree();
    tree.walk(&mut PrintTreeListing { depth: 0 });

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(300.0, 500.0),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(EguiPlugin)
        .add_systems(Update, move |ctx: EguiContexts| {
            egui(ctx, &mut tree);
        })
        .add_systems(Startup, |mut _ctx: EguiContexts| {
            //dear_egui::set_theme(_ctx.ctx_mut(), dear_egui::SKY);
        })
        .run();
}

fn egui(mut ctx: EguiContexts, tree: &mut Node) {
    egui::CentralPanel::default().show(ctx.ctx_mut(), |ui| {
        let res = TreeViewBuilder::new(ui, ui.make_persistent_id("tree view"), |root| {
            tree.walk(&mut TreeViewVisitor { builder: root });
        });

        if let Some(drop_action) = res.drag_drop_action {
            // Test if drop is valid
            let mut drop_allowed_visitor =
                DropAllowedVisitor::new(drop_action.drag_id, drop_action.drop_id, &tree);
            tree.walk(&mut drop_allowed_visitor);

            if drop_allowed_visitor.is_drop_allowed() {
                // remove dragged node
                let mut remove_visitor = RemoveNodeVisitor::new(drop_action.drag_id);
                tree.walk_mut(&mut remove_visitor);

                // insert node
                if let Some(dragged_node) = remove_visitor.removed_node {
                    tree.walk_mut(&mut InsertNodeVisitor {
                        target_id: drop_action.drop_id,
                        position: drop_action.position,
                        node: Some(dragged_node),
                    });
                }
            }
        }
    });
}
