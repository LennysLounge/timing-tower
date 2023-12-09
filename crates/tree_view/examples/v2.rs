use bevy::{
    ecs::{query::With, system::Query},
    prelude::{App, Commands, ResMut, Resource, Startup, Update},
    window::{PrimaryWindow, Window},
    DefaultPlugins,
};
use bevy_egui::{
    egui::{self},
    EguiContexts, EguiPlugin,
};

use tree_view::{v2::TreeViewBuilder, DropPosition};
use uuid::{uuid, Uuid};

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

fn setup(
    mut commands: Commands,
    mut _ctx: EguiContexts,
    mut window: Query<&mut Window, With<PrimaryWindow>>,
) {
    window.single_mut().resolution.set(300.0, 500.0);
    let mut root = Directory::new(
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
            File::new_with_id("ABAB", uuid!("5ef68c19-45fd-4d34-84b5-89948df109f9")),
            File::new("BBBB"),
            Directory::new("Dodads", vec![File::new("EEEE"), File::new("FFFF")]),
        ],
    );

    root.accept(&mut PrintVisitor {
        depth: 0,
        stop_at: uuid!("5ef68c19-45fd-4d34-84b5-89948df109f9"),
    });

    commands.insert_resource(EditorState { tree: root });
    //dear_egui::set_theme(ctx.ctx_mut(), dear_egui::SKY);
}

fn egui(mut ctx: EguiContexts, mut state: ResMut<EditorState>) {
    egui::CentralPanel::default().show(ctx.ctx_mut(), |ui| {
        let EditorState { tree } = &mut *state;

        let res = TreeViewBuilder::new(ui, ui.make_persistent_id("tree view"), |root| {
            tree.accept(&mut TreeViewVisitor::new(root));
        });

        // for action in res.inner.into_iter() {
        //     match action {
        //         tree_view::v2::TreeViewAction::Drop {
        //             node_to_remove,
        //             receiver_node,
        //             position,
        //         } => {
        //             let mut remove_visitor = RemoveNodeVisitor::new(node_to_remove);
        //             tree.accept(&mut remove_visitor);
        //             if let Some(node) = remove_visitor.removed_node {
        //                 tree.accept(&mut InsertNodeVisitor {
        //                     receiver_node,
        //                     position,
        //                     node: Some(node),
        //                 });
        //             }
        //         }
        //     }
        // }
    });
}

enum Node {
    Directory(Directory),
    File(File),
}
impl Node {
    fn id(&self) -> &Uuid {
        match self {
            Node::Directory(d) => &d.id,
            Node::File(d) => &d.id,
        }
    }
}
impl Visitable for Node {
    fn accept(&mut self, visitor: &mut dyn NodeVisitor) -> bool {
        match self {
            Node::Directory(d) => d.accept(visitor),
            Node::File(f) => f.accept(visitor),
        }
    }
}

struct Directory {
    id: Uuid,
    name: String,
    nodes: Vec<Node>,
    _is_root: bool,
}

impl Directory {
    fn new(name: &str, nodes: Vec<Node>) -> Node {
        Node::Directory(Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            nodes,
            _is_root: false,
        })
    }
}
impl Visitable for Directory {
    fn accept(&mut self, visitor: &mut dyn NodeVisitor) -> bool {
        if visitor.enter_dir(self) {
            self.nodes.iter_mut().all(|n| n.accept(visitor));
        }
        visitor.leave_dir(self)
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
    fn new_with_id(name: &str, id: Uuid) -> Node {
        Node::File(Self {
            id,
            name: name.to_string(),
        })
    }
}
impl Visitable for File {
    fn accept(&mut self, visitor: &mut dyn NodeVisitor) -> bool {
        visitor.visit_file(self)
    }
}

trait NodeVisitor {
    fn enter_dir(&mut self, _dir: &mut Directory) -> bool {
        true
    }
    fn leave_dir(&mut self, _dir: &mut Directory) -> bool {
        true
    }
    fn visit_file(&mut self, _file: &mut File) -> bool {
        true
    }
}

trait Visitable {
    fn accept(&mut self, visitor: &mut dyn NodeVisitor) -> bool;
}

struct PrintVisitor {
    stop_at: Uuid,
    depth: usize,
}
impl NodeVisitor for PrintVisitor {
    fn enter_dir(&mut self, dir: &mut Directory) -> bool {
        if self.stop_at == dir.id {
            return false;
        }

        println!("{:>depth$} Dir '{}'", "", dir.name, depth = self.depth);
        self.depth += 4;
        true
    }

    fn leave_dir(&mut self, _dir: &mut Directory) -> bool {
        self.depth -= 4;
        true
    }

    fn visit_file(&mut self, file: &mut File) -> bool {
        if self.stop_at == file.id {
            return false;
        }
        println!("{:>depth$} File '{}'", "", file.name, depth = self.depth);
        true
    }
}

struct TreeViewVisitor<'a> {
    builder: TreeViewBuilder<'a>,
}
impl<'a> TreeViewVisitor<'a> {
    fn new(builder: TreeViewBuilder<'a>) -> Self {
        Self { builder: builder }
    }
}
impl NodeVisitor for TreeViewVisitor<'_> {
    fn enter_dir(&mut self, dir: &mut Directory) -> bool {
        let res = self.builder.dir(&dir.id, |ui| {
            ui.label(&dir.name);
        });
        if let Some(res) = res{
            res.context_menu(|ui|{
                ui.label("Contex menu of a dir");
            });
        }
        true
    }

    fn leave_dir(&mut self, _dir: &mut Directory) -> bool {
        self.builder.close_dir();
        true
    }

    fn visit_file(&mut self, file: &mut File) -> bool {
        let res = self.builder.leaf(&file.id, |ui| {
            ui.label(&file.name);
        });
        if let Some(res) = res{
            res.context_menu(|ui|{
                ui.label("Contex menu of a leaf");
            });
        }
        true
    }
}

struct RemoveNodeVisitor {
    id: Uuid,
    removed_node: Option<Node>,
}
impl RemoveNodeVisitor {
    fn new(id: Uuid) -> Self {
        Self {
            id,
            removed_node: None,
        }
    }
}
impl NodeVisitor for RemoveNodeVisitor {
    fn enter_dir(&mut self, dir: &mut Directory) -> bool {
        if let Some(index) = dir.nodes.iter().position(|n| n.id() == &self.id) {
            self.removed_node = Some(dir.nodes.remove(index));
            false
        } else {
            true
        }
    }
}

struct InsertNodeVisitor {
    receiver_node: Uuid,
    position: DropPosition,
    node: Option<Node>,
}
impl NodeVisitor for InsertNodeVisitor {
    fn enter_dir(&mut self, dir: &mut Directory) -> bool {
        if self.receiver_node != dir.id {
            return true;
        }
        let node = self.node.take().unwrap();
        match self.position {
            DropPosition::First => dir.nodes.insert(0, node),
            DropPosition::Last => dir.nodes.push(node),
            DropPosition::After(id) => {
                if let Some(position) = dir.nodes.iter().position(|n| n.id() == &id) {
                    dir.nodes.insert(position + 1, node);
                }
            }
            DropPosition::Before(id) => {
                if let Some(position) = dir.nodes.iter().position(|n| n.id() == &id) {
                    dir.nodes.insert(position, node);
                }
            }
        }
        false
    }
}
