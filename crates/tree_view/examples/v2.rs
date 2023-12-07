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

use tree_view::v2::TreeViewBuilder2;
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
            File::new("ABAB"),
            File::new("BBBB"),
            Directory::new("Dodads", vec![File::new("EEEE"), File::new("FFFF")]),
        ],
    );

    root.accept(&mut PrintVisitor { depth: 0 });

    commands.insert_resource(EditorState { tree: root });
    //dear_egui::set_theme(ctx.ctx_mut(), dear_egui::SKY);
}

fn egui(mut ctx: EguiContexts, mut state: ResMut<EditorState>) {
    egui::CentralPanel::default().show(ctx.ctx_mut(), |ui| {
        let EditorState { tree } = &mut *state;

        TreeViewBuilder2::new(ui, ui.make_persistent_id("tree view"), |root| {
            tree.accept(&mut TreeViewVisitor::new(root));
        });
    });
}

enum Node {
    Directory(Directory),
    File(File),
}
impl Visitable for Node {
    fn accept(&mut self, visitor: &mut dyn NodeVisitor) {
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
    fn accept(&mut self, visitor: &mut dyn NodeVisitor) {
        visitor.enter_dir(self);
        self.nodes.iter_mut().for_each(|n| match n {
            Node::Directory(d) => d.accept(visitor),
            Node::File(f) => f.accept(visitor),
        });
        visitor.leave_dir(self);
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
}
impl Visitable for File {
    fn accept(&mut self, visitor: &mut dyn NodeVisitor) {
        visitor.visit_file(self);
    }
}

trait NodeVisitor {
    fn enter_dir(&mut self, dir: &mut Directory);
    fn leave_dir(&mut self, dir: &mut Directory);
    fn visit_file(&mut self, file: &mut File);
}

trait Visitable {
    fn accept(&mut self, visitor: &mut dyn NodeVisitor);
}

struct PrintVisitor {
    depth: usize,
}
impl NodeVisitor for PrintVisitor {
    fn enter_dir(&mut self, dir: &mut Directory) {
        println!("{:>depth$} Dir '{}'", "", dir.name, depth = self.depth);
        self.depth += 4;
    }

    fn leave_dir(&mut self, _dir: &mut Directory) {
        self.depth -= 4;
    }

    fn visit_file(&mut self, file: &mut File) {
        println!("{:>depth$} File '{}'", "", file.name, depth = self.depth);
    }
}

struct TreeViewVisitor<'a> {
    builder: TreeViewBuilder2<'a>,
}
impl<'a> TreeViewVisitor<'a> {
    fn new(builder: TreeViewBuilder2<'a>) -> Self {
        Self { builder: builder }
    }
}
impl NodeVisitor for TreeViewVisitor<'_> {
    fn enter_dir(&mut self, dir: &mut Directory) {
        self.builder.dir(&dir.id, |ui| {
            ui.label(format!("{} {}",&dir.name, &dir.id));
            //ui.label(&dir.name);
        });
    }

    fn leave_dir(&mut self, _dir: &mut Directory) {
        self.builder.close_dir();
    }

    fn visit_file(&mut self, file: &mut File) {
        self.builder.leaf(&file.id, |ui| {
            ui.label(format!("{} {}",&file.name, &file.id));
            //ui.label(&file.name);
        });
    }
}
