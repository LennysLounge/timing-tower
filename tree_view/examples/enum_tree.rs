use bevy::{
    prelude::{App, Commands, ResMut, Resource, Startup, Update},
    DefaultPlugins,
};
use bevy_egui::{
    egui::{self, epaint, LayerId, Order, Sense, Shape, Ui},
    EguiContexts, EguiPlugin,
};
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

fn setup(mut commands: Commands) {
    let root = Node::new_dir(
        "Root",
        vec![
            Node::new_dir(
                "Things",
                vec![
                    Node::new_dir("Other things", vec![Node::new_file("GGGG")]),
                    Node::new_file("CCCC"),
                    Node::new_file("DDDD"),
                ],
            ),
            Node::new_file("AAAA"),
            Node::new_file("BBBB"),
            Node::new_dir(
                "Dodads",
                vec![Node::new_file("EEEE"), Node::new_file("FFFF")],
            ),
        ],
    );

    commands.insert_resource(EditorState { tree: root });
}

fn egui(mut ctx: EguiContexts, mut state: ResMut<EditorState>) {
    egui::CentralPanel::default().show(ctx.ctx_mut(), |ui| {
        let mut hovered = None;
        let mut dragged = None;
        state.tree.show(ui, &mut hovered, &mut dragged);

        ui.label(format!("hovered: {:?}", hovered));
        ui.label(format!("dragged: {:?}", dragged));

        let drag_released = ui.input(|i| i.pointer.any_released());
        if let (Some(target_id), Some(node_id), true) = (hovered, dragged, drag_released) {
            if let Some(node) = state.tree.remove(&node_id) {
                if let Some(target_node) = state.tree.find_mut(&target_id) {
                    target_node.insert(node);
                } else {
                    println!("How can there not be a target node?");
                }
            } else {
                println!("how can there not be a dragged node");
            }
        }
    });
}

enum Node {
    Directory(Uuid, String, Vec<Node>),
    File(Uuid, String),
}

impl Node {
    fn new_dir(name: &str, nodes: Vec<Node>) -> Self {
        Self::Directory(Uuid::new_v4(), name.to_string(), nodes)
    }

    fn new_file(name: &str) -> Self {
        Self::File(Uuid::new_v4(), name.to_string())
    }

    fn find_mut(&mut self, search_id: &Uuid) -> Option<&mut Node> {
        match self {
            Node::Directory(id, _, _) => {
                if id == search_id {
                    Some(self)
                } else {
                    match self {
                        Node::Directory(_, _, nodes) => {
                            nodes.iter_mut().find_map(|n| n.find_mut(search_id))
                        }
                        _ => None,
                    }
                }
            }
            Node::File(id, _) => {
                if id == search_id {
                    Some(self)
                } else {
                    None
                }
            }
        }
    }

    fn remove(&mut self, remove_id: &Uuid) -> Option<Node> {
        match self {
            Node::Directory(_, _, nodes) => {
                let pos = nodes.iter().position(|n| n.id() == remove_id);
                if let Some(pos) = pos {
                    Some(nodes.remove(pos))
                } else {
                    nodes.iter_mut().find_map(|n| n.remove(remove_id))
                }
            }
            Node::File(_, _) => None,
        }
    }

    fn insert(&mut self, node: Node) {
        match self {
            Node::Directory(_, _, nodes) => nodes.push(node),
            Node::File(_, _) => (),
        }
    }

    fn id(&self) -> &Uuid {
        match self {
            Node::Directory(id, _, _) => id,
            Node::File(id, _) => id,
        }
    }

    fn show(&self, ui: &mut Ui, hovered: &mut Option<Uuid>, dragged: &mut Option<Uuid>) {
        match self {
            Node::Directory(id, name, nodes) => {
                let where_to_put_background = ui.painter().add(Shape::Noop);

                let mut content_ui = ui.child_ui(ui.available_rect_before_wrap(), *ui.layout());
                let _res = content_ui.collapsing(name, |ui| {
                    nodes.iter().for_each(|n| n.show(ui, hovered, dragged));
                });
                let content_rect = content_ui.min_rect();

                let (rect, response) = ui.allocate_at_least(content_rect.size(), Sense::hover());
                let is_hovered = response.hovered() && hovered.is_none();
                if is_hovered {
                    *hovered = Some(*id);
                }

                let style = if is_hovered {
                    ui.visuals().widgets.active
                } else {
                    ui.visuals().widgets.inactive
                };

                ui.painter().set(
                    where_to_put_background,
                    epaint::RectShape {
                        rect,
                        rounding: style.rounding,
                        fill: style.bg_fill,
                        stroke: style.bg_stroke,
                    },
                );
            }
            Node::File(id, name) => {
                let body = |ui: &mut Ui| {
                    ui.label(name);
                };

                let drag_id = ui.next_auto_id();
                let is_being_dragged = ui.memory(|mem| mem.is_being_dragged(drag_id));

                if !is_being_dragged {
                    let res = ui.scope(body).response;
                    ui.interact(res.rect, drag_id, Sense::drag());
                } else {
                    *dragged = Some(*id);
                    ui.ctx().set_cursor_icon(egui::CursorIcon::Grabbing);

                    // Paint the body to a new layer:
                    let layer_id = LayerId::new(Order::Tooltip, drag_id);
                    let response = ui.with_layer_id(layer_id, body).response;

                    // Now we move the visuals of the body to where the mouse is.
                    // Normally you need to decide a location for a widget first,
                    // because otherwise that widget cannot interact with the mouse.
                    // However, a dragged component cannot be interacted with anyway
                    // (anything with `Order::Tooltip` always gets an empty [`Response`])
                    // So this is fine!

                    if let Some(pointer_pos) = ui.ctx().pointer_interact_pos() {
                        let delta = pointer_pos - response.rect.center();
                        ui.ctx().translate_layer(layer_id, delta);
                    }
                    //ui.scope(body);
                }
            }
        }
    }
}
