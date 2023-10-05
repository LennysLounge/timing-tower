use bevy::{
    prelude::{App, Commands, ResMut, Resource, Startup, Update},
    DefaultPlugins,
};
use bevy_egui::{
    egui::{self},
    EguiContexts, EguiPlugin,
};
use tree_view::tree_view_2::{self, TreeUi, TreeView};
use uuid::Uuid;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_systems(Update, egui)
        .add_systems(Startup, setup)
        .run();
}

#[derive(Resource)]
struct EditorState {
    tree: Node,
}

fn setup(mut commands: Commands, mut _ctx: EguiContexts) {
    let root = Directory::new(
        "Root",
        vec![
            Directory::new(
                "Things",
                vec![
                    Directory::new("Other things", vec![File::new("GGGG")]),
                    File::new("CCCC"),
                    File::new("DDDD"),
                ],
            ),
            File::new("AAAA"),
            File::new("ABAB"),
            File::new("BBBB"),
            Directory::new("Dodads", vec![File::new("EEEE"), File::new("FFFF")]),
        ],
    );

    commands.insert_resource(EditorState { tree: root });

    //dear_egui::set_theme(ctx.ctx_mut(), dear_egui::SKY);
}

fn egui(mut ctx: EguiContexts, mut state: ResMut<EditorState>) {
    egui::SidePanel::left("left panel").show(ctx.ctx_mut(), |ui| {
        let EditorState { tree: _ } = &mut *state;

        let res = TreeView::new().show(ui, |tree_ui| {
            state.tree.show_tree(tree_ui);
        });

        ui.allocate_space(ui.available_size_before_wrap());
    });
}

enum Node {
    Directory(Directory),
    File(File),
}

impl Node {
    fn show_tree(&self, tree_ui: &mut TreeUi) {
        match self {
            Node::Directory(o) => o.show(tree_ui),
            Node::File(o) => o.show(tree_ui),
        }
    }
}

struct Directory {
    id: Uuid,
    name: String,
    nodes: Vec<Node>,
}

impl Directory {
    fn new(name: &str, nodes: Vec<Node>) -> Node {
        Node::Directory(Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            nodes,
        })
    }

    fn show(&self, tree_ui: &mut TreeUi) {
        let (header, _) = tree_view_2::Directory::new(self.id).show(
            tree_ui,
            |ui| {
                ui.label(&self.name);
            },
            |ui| {
                for node in self.nodes.iter() {
                    node.show_tree(ui);
                }
            },
        );
        if header.response.clicked() {
            println!("{} was clicked", self.name);
        }
        header.response.context_menu(|ui| {
            ui.label(format!("context menu for {}", self.name));
        });
    }
}

struct File {
    id: Uuid,
    name: String,
}

impl File {
    fn new(name: &str) -> Node {
        Node::File(Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
        })
    }
    fn show(&self, tree_ui: &mut TreeUi) {
        let res = tree_view_2::Leaf::new(self.id).show(tree_ui, |ui| {
            ui.label(&self.name);
        });

        if res.response.clicked() {
            println!("{} was clicked", self.name);
        }
        res.response.context_menu(|ui| {
            ui.label(format!("context menu for {}", self.name));
        });
    }
}
