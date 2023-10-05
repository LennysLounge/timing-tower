use std::any::Any;

use bevy::{
    prelude::{App, Commands, ResMut, Resource, Startup, Update},
    DefaultPlugins,
};
use bevy_egui::{
    egui::{self},
    EguiContexts, EguiPlugin,
};
use tree_view::tree_view_2::{self, DropAction, DropPosition, TreeUi, TreeView};
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

        if let Some(DropAction { from, to }) = res.dropped {
            if let Some(node) = state
                .tree
                .find_mut(&from.parent_id)
                .and_then(|node| node.remove(&from.node_id))
            {
                state
                    .tree
                    .find_mut(to.get_parent_node_id())
                    .map(|parent| parent.insert(node, to));
            }
        }

        ui.label("Selected:");
        let name = res
            .selected
            .and_then(|selected_id| state.tree.find(&selected_id))
            .map(|n| n.name().as_str())
            .unwrap_or("----");
        ui.label(name);

        ui.allocate_space(ui.available_size_before_wrap());
    });
}

trait TreeNode: Any {
    fn name(&self) -> &String;
    fn find(&self, id: &Uuid) -> Option<&dyn TreeNode>;
    fn find_mut(&mut self, id: &Uuid) -> Option<&mut dyn TreeNode>;
    fn remove(&mut self, id: &Uuid) -> Option<Node>;
    fn insert(&mut self, node: Node, position: DropPosition);
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

    fn id(&self) -> &Uuid {
        match self {
            Node::Directory(o) => o.id(),
            Node::File(o) => o.id(),
        }
    }
}

impl TreeNode for Node {
    fn find(&self, id: &Uuid) -> Option<&dyn TreeNode> {
        match self {
            Node::Directory(o) => o.find(id),
            Node::File(o) => o.find(id),
        }
    }

    fn find_mut(&mut self, id: &Uuid) -> Option<&mut dyn TreeNode> {
        match self {
            Node::Directory(o) => o.find_mut(id),
            Node::File(o) => o.find_mut(id),
        }
    }

    fn name(&self) -> &String {
        match self {
            Node::Directory(o) => o.name(),
            Node::File(o) => o.name(),
        }
    }

    fn remove(&mut self, id: &Uuid) -> Option<Node> {
        match self {
            Node::Directory(o) => o.remove(id),
            Node::File(o) => o.remove(id),
        }
    }

    fn insert(&mut self, node: Node, position: DropPosition) {
        match self {
            Node::Directory(o) => o.insert(node, position),
            Node::File(o) => o.insert(node, position),
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
    fn id(&self) -> &Uuid {
        &self.id
    }
}

impl TreeNode for Directory {
    fn find(&self, id: &Uuid) -> Option<&dyn TreeNode> {
        if &self.id == id {
            Some(self as &dyn TreeNode)
        } else {
            self.nodes.iter().find_map(|n| n.find(id))
        }
    }
    fn find_mut(&mut self, id: &Uuid) -> Option<&mut dyn TreeNode> {
        if &self.id == id {
            Some(self as &mut dyn TreeNode)
        } else {
            self.nodes.iter_mut().find_map(|n| n.find_mut(id))
        }
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn remove(&mut self, id: &Uuid) -> Option<Node> {
        if let Some(pos) = self.nodes.iter().position(|n| n.id() == id) {
            Some(self.nodes.remove(pos))
        } else {
            None
        }
    }

    fn insert(&mut self, node: Node, position: DropPosition) {
        let pos = match position {
            DropPosition::Last { .. } => self.nodes.len(),
            DropPosition::First { .. } => 0,
            DropPosition::After { after, .. } => {
                self.nodes
                    .iter()
                    .position(|n| n.id() == &after)
                    .unwrap_or(self.nodes.len())
                    + 1
            }
            DropPosition::Before { before, .. } => self
                .nodes
                .iter()
                .position(|n| n.id() == &before)
                .unwrap_or(self.nodes.len()),
        };
        self.nodes.insert(pos, node);
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
    fn id(&self) -> &Uuid {
        &self.id
    }
}

impl TreeNode for File {
    fn find(&self, id: &Uuid) -> Option<&dyn TreeNode> {
        if &self.id == id {
            Some(self as &dyn TreeNode)
        } else {
            None
        }
    }

    fn find_mut(&mut self, id: &Uuid) -> Option<&mut dyn TreeNode> {
        if &self.id == id {
            Some(self as &mut dyn TreeNode)
        } else {
            None
        }
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn remove(&mut self, _id: &Uuid) -> Option<Node> {
        None
    }

    fn insert(&mut self, _node: Node, _position: DropPosition) {}
}
