use bevy::{
    prelude::{App, Commands, ResMut, Resource, Startup, Update},
    DefaultPlugins,
};
use bevy_egui::{
    egui::{self, Ui},
    EguiContexts, EguiPlugin,
};
use tree_view::{TreeNode, TreeView};
use uuid::Uuid;

mod tree_view;

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
    tree_view: TreeView,
}

fn setup(mut commands: Commands, mut _ctx: EguiContexts) {
    let root = Directory::new_box(
        "Root",
        vec![
            Directory::new_box(
                "Things",
                vec![
                    Directory::new_box("Other things", vec![File::new_box("GGGG")]),
                    File::new_box("CCCC"),
                    File::new_box("DDDD"),
                ],
            ),
            File::new_box("AAAA"),
            File::new_box("BBBB"),
            Directory::new_box("Dodads", vec![File::new_box("EEEE"), File::new_box("FFFF")]),
        ],
    );

    commands.insert_resource(EditorState {
        tree: root,
        tree_view: TreeView { selected: None },
    });

    //dear_egui::set_theme(ctx.ctx_mut(), dear_egui::SKY);
}

fn egui(mut ctx: EguiContexts, mut state: ResMut<EditorState>) {
    egui::SidePanel::left("left panel").show(ctx.ctx_mut(), |ui| {
        let EditorState { tree, tree_view } = &mut *state;

        tree_view.show(ui, tree);
        ui.label("After tree view");

        ui.allocate_space(ui.available_size_before_wrap());
    });
}

enum Node {
    Directory(Directory),
    File(File),
}

impl TreeNode for Node {
    fn is_directory(&self) -> bool {
        match self {
            Node::Directory(_) => true,
            Node::File(_) => false,
        }
    }

    fn show(&self, ui: &mut egui::Ui) {
        match self {
            Node::Directory(d) => d.show(ui),
            Node::File(f) => f.show(ui),
        }
    }

    fn get_children(&self) -> Vec<&dyn TreeNode> {
        match self {
            Node::Directory(dir) => dir.nodes.iter().map(|n| n as &dyn TreeNode).collect(),
            Node::File(_) => Vec::new(),
        }
    }

    fn get_id(&self) -> &Uuid {
        match self {
            Node::Directory(d) => &d.id,
            Node::File(f) => &f.id,
        }
    }
}

struct Directory {
    id: Uuid,
    name: String,
    nodes: Vec<Node>,
}

impl Directory {
    fn new(name: &str, nodes: Vec<Node>) -> Directory {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            nodes,
        }
    }
    fn new_box(name: &str, nodes: Vec<Node>) -> Node {
        Node::Directory(Self::new(name, nodes))
    }

    fn show(&self, ui: &mut Ui) {
        ui.label(&self.name);
    }
}

struct File {
    id: Uuid,
    name: String,
}

impl File {
    fn new(name: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
        }
    }
    fn new_box(name: &str) -> Node {
        Node::File(Self::new(name))
    }

    fn show(&self, ui: &mut Ui) {
        ui.label(&self.name);
    }
}
