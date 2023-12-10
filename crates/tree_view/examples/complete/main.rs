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
use visitor::TreeViewVisitor;

mod visitor;

fn main() {
    let mut tree = make_tree();

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
            TreeViewVisitor::run(tree, root);
        });

        if let Some(_drop_action) = res.drag_drop_action {
            // Test if drop is valid

            // remove dragged node

            // insert node
        }

        // for action in res.inner.into_iter() {
        //     match action {
        //         tree_view::v2::TreeViewAction::Drop {
        //             node_to_remove,
        //             receiver_node,
        //             position,
        //         } => {
        //             // let mut remove_visitor = RemoveNodeVisitor::new(node_to_remove);
        //             // tree.accept(&mut remove_visitor);
        //             // if let Some(node) = remove_visitor.removed_node {
        //             //     tree.accept(&mut InsertNodeVisitor {
        //             //         receiver_node,
        //             //         position,
        //             //         node: Some(node),
        //             //     });
        //             // }
        //         }
        //     }
        // }
    });
}
