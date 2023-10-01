use bevy_egui::egui::{
    self,
    epaint::{self, RectShape},
    pos2, vec2, Color32, Id, InnerResponse, LayerId, Order, Rect, Rounding, Sense, Shape, Stroke,
    Ui, Vec2,
};
use uuid::Uuid;

use crate::split_collapsing_state::SplitCollapsingState;

#[derive(Debug, Clone)]
pub enum DropAction {
    On { parent_id: Uuid },
    After { parent_id: Uuid, child_id: Uuid },
    Before { parent_id: Uuid, child_id: Uuid },
}
impl DropAction {
    fn get_parent_node_id(&self) -> &Uuid {
        match self {
            DropAction::On { parent_id } => parent_id,
            DropAction::After { parent_id, .. } => parent_id,
            DropAction::Before { parent_id, .. } => parent_id,
        }
    }
}

pub trait TreeNode {
    type NodeType;

    fn is_directory(&self) -> bool;
    fn show(&self, ui: &mut Ui);
    fn get_id(&self) -> &Uuid;
    fn get_children(&self) -> Vec<&dyn TreeNode<NodeType = Self::NodeType>>;
    fn get_children_mut(&mut self) -> Vec<&mut dyn TreeNode<NodeType = Self::NodeType>>;
    fn as_trait(&self) -> &dyn TreeNode<NodeType = Self::NodeType>;
    fn as_trait_mut(&mut self) -> &mut dyn TreeNode<NodeType = Self::NodeType>;
    fn remove_child(&mut self, id: &Uuid) -> Option<Self::NodeType>;
    fn insert(&mut self, drop_action: &DropAction, node: Self::NodeType);

    fn find(&self, id: &Uuid) -> Option<&dyn TreeNode<NodeType = Self::NodeType>> {
        if self.get_id() == id {
            Some(self.as_trait())
        } else {
            self.get_children().iter().find_map(|n| n.find(id))
        }
    }

    fn find_mut(&mut self, id: &Uuid) -> Option<&mut dyn TreeNode<NodeType = Self::NodeType>> {
        if self.get_id() == id {
            Some(self.as_trait_mut())
        } else {
            self.get_children_mut()
                .into_iter()
                .find_map(|n| n.find_mut(id))
        }
    }

    fn find_parent_mut(
        &mut self,
        child_id: &Uuid,
    ) -> Option<&mut dyn TreeNode<NodeType = Self::NodeType>> {
        let has_child = self
            .get_children_mut()
            .into_iter()
            .any(|c| c.get_id() == child_id);
        if has_child {
            Some(self.as_trait_mut())
        } else {
            self.get_children_mut()
                .into_iter()
                .find_map(|c| c.find_parent_mut(child_id))
        }
    }

    fn remove(&mut self, id: &Uuid) -> Option<Self::NodeType> {
        if let Some(parent) = self.find_parent_mut(id) {
            parent.remove_child(id)
        } else {
            None
        }
    }

    fn drop(&mut self, drop_action: &DropAction, node: Self::NodeType) {
        if let Some(parent) = self.find_mut(drop_action.get_parent_node_id()) {
            parent.insert(&drop_action, node);
        }
    }
}

pub const DRAG_LINE_HEIGHT: f32 = 3.0;
pub const DRAG_LINE_HOVER_HEIGHT: f32 = 5.0;

pub struct TreeView {
    pub selected: Option<Uuid>,
    pub was_dragged_last_frame: Option<Uuid>,
}

impl TreeView {
    pub fn show<N: TreeNode>(&mut self, ui: &mut Ui, root: &mut dyn TreeNode<NodeType = N>) {
        let mut context = TreeViewContext {
            was_dragged_last_frame: self.was_dragged_last_frame,
            bounds: ui.available_rect_before_wrap(),
            line_count: 0,
            dragged: None,
            hovered: None,
            node: root,
            selected: self.selected,
            parent: None,
        };

        let res = ui.allocate_ui(ui.available_size_before_wrap(), |ui| {
            ui.style_mut().spacing.item_spacing.y = 7.0;
            // Allocate a bit of space to add half of one item spacing worth of space.
            // Allocating normals adds a full space so we take away half.
            ui.allocate_at_least(
                vec2(0.0, -ui.spacing().item_spacing.y / 2.0),
                Sense::hover(),
            );
            context.show_node(ui);
            // Allocate a bit of space to add half of one item spacing worth of space.
            // Allocating normals adds a full space so we take away half.
            ui.allocate_at_least(
                vec2(0.0, -ui.spacing().item_spacing.y / 2.0),
                Sense::hover(),
            );
        });

        let TreeViewContext {
            dragged,
            hovered,
            selected,
            ..
        } = context;

        self.selected = selected;
        self.was_dragged_last_frame = dragged;

        ui.painter().rect_stroke(
            res.response.rect,
            Rounding::none(),
            Stroke::new(1.0, Color32::BLACK),
        );

        // Move the node to the drag target.
        let drag_released = ui.input(|i| i.pointer.any_released());
        if let (Some(drop_action), Some(drag_source), true) = (&hovered, &dragged, drag_released) {
            // The node we are dropping on cannot be a child of the node we are dragging.
            if root
                .find(drag_source)
                .and_then(|source| source.find(drop_action.get_parent_node_id()))
                .is_some()
            {
                println!("Cannot drop a parent into its child");
            } else {
                if let Some(node) = root.remove(drag_source) {
                    println!("Removed: {:?}", node.get_id());
                    root.drop(drop_action, node);
                    println!("Place {:?}", drop_action);
                }
            }
        }

        ui.label(format!("Dragged: {:?}", dragged));
        ui.label(format!("Hovered: {:?}", hovered));
    }
}

struct TreeViewContext<'a, N> {
    node: &'a dyn TreeNode<NodeType = N>,
    parent: Option<Uuid>,
    bounds: Rect,
    line_count: i32,
    selected: Option<Uuid>,
    was_dragged_last_frame: Option<Uuid>,
    dragged: Option<Uuid>,
    hovered: Option<DropAction>,
}

struct ShowNodeResult<T> {
    rect: Rect,
    clicked: bool,
    dragged: bool,
    inner: T,
}

enum DropTargetPos {
    Upper,
    Lower,
    Above,
    Below,
    On,
}

impl<'a, N> TreeViewContext<'a, N> {
    fn show_node(&mut self, ui: &mut Ui) {
        let mut child_ui = ui.child_ui_with_id_source(
            ui.available_rect_before_wrap(),
            *ui.layout(),
            self.node.get_id(),
        );
        self.show_node_ui(&mut child_ui);
        ui.allocate_at_least(child_ui.min_rect().size(), Sense::hover());
    }

    fn show_node_ui(&mut self, ui: &mut Ui) {
        let where_to_put_background = ui.painter().add(Shape::Noop);

        self.line_count += 1;
        let is_even = self.line_count % 2 == 0;
        let is_selected = self.selected.is_some_and(|id| &id == self.node.get_id());

        let node_result = if self.node.is_directory() {
            let result = self.dir_drop_target(ui, |me, ui| {
                let result = me.drag_source(ui, |me, ui| me.show_dir_header(ui));
                me.show_dir_body(ui, &result.inner);
                (result.rect, result)
            });
            ShowNodeResult {
                rect: result.rect,
                clicked: result.clicked,
                dragged: result.dragged,
                inner: (),
            }
        } else {
            let result = self.leaf_drop_target(ui, |me, ui| {
                let result = me.drag_source(ui, |me, ui| me.show_leaf(ui));
                (result.rect, result)
            });

            result
        };

        if node_result.clicked || node_result.dragged {
            self.selected = Some(*self.node.get_id());
        }

        ui.painter().set(
            where_to_put_background,
            epaint::RectShape {
                rect: node_result.rect,
                rounding: ui.visuals().widgets.active.rounding,
                fill: if is_selected {
                    ui.style().visuals.selection.bg_fill
                } else if is_even {
                    Color32::from_rgba_premultiplied(10, 10, 10, 0)
                } else {
                    Color32::TRANSPARENT
                },
                stroke: Stroke::NONE,
            },
        );
    }

    fn check_drop_target(
        ui: &mut Ui,
        hover_rect: Rect,
        content_rect: Rect,
        drop_pos: DropTargetPos,
    ) -> (bool, RectShape) {
        //let content_rect = hover_rect;
        let (height, dir, line_height) = match drop_pos {
            DropTargetPos::Upper => (hover_rect.height() / 2.0, -1.0, DRAG_LINE_HEIGHT),
            DropTargetPos::Lower => (hover_rect.height() / 2.0, 1.0, DRAG_LINE_HEIGHT),
            DropTargetPos::Above => (DRAG_LINE_HOVER_HEIGHT, -1.0, DRAG_LINE_HEIGHT),
            DropTargetPos::Below => (DRAG_LINE_HOVER_HEIGHT, 1.0, DRAG_LINE_HEIGHT),
            DropTargetPos::On => (
                hover_rect.height() - DRAG_LINE_HOVER_HEIGHT * 2.0,
                0.0,
                hover_rect.height(),
            ),
        };

        let drop_rect = Rect::from_center_size(
            hover_rect.center() + vec2(0.0, (hover_rect.height() - height) * dir / 2.0),
            vec2(hover_rect.width(), height),
        );
        // When checking for interaction, egui adds the item spacing on to. To get
        // a clean break we take it away again.
        let drop_rect = drop_rect.expand2(vec2(0.0, -ui.spacing().item_spacing.y / 4.0));

        // ui.painter().rect_filled(
        //     drop_rect,
        //     Rounding::none(),
        //     match dir {
        //         ref x if *x < -0.5 => Color32::RED,
        //         ref x if *x > 0.5 => Color32::BLUE,
        //         _ => Color32::GREEN,
        //     }
        //     .linear_multiply(0.05),
        // );

        let interaction = ui.interact(drop_rect, ui.next_auto_id(), Sense::hover());

        let rect = if interaction.hovered {
            Rect::from_center_size(
                pos2(
                    content_rect.center().x,
                    hover_rect.center().y + hover_rect.height() * dir / 2.0,
                ),
                vec2(content_rect.width(), line_height),
            )
        } else {
            Rect::NOTHING
        };
        let shape = epaint::RectShape {
            rect,
            rounding: ui.visuals().widgets.active.rounding,
            fill: ui.visuals().selection.bg_fill,
            stroke: Stroke::NONE,
        };
        (interaction.hovered(), shape)
    }

    fn dir_drop_target<T>(
        &mut self,
        ui: &mut Ui,
        mut add_content: impl FnMut(&mut Self, &mut Ui) -> (Rect, T),
    ) -> T {
        let where_to_put_background = ui.painter().add(Shape::Noop);

        let InnerResponse {
            inner: (row_rect, result),
            response,
        } = ui.allocate_ui(ui.available_size_before_wrap(), |ui| add_content(self, ui));

        if self
            .was_dragged_last_frame
            .map_or(true, |id| &id == self.node.get_id())
        {
            return result;
        }

        if let Some(parent_id) = self.parent {
            let (upper_hovered, shape) =
                Self::check_drop_target(ui, row_rect, response.rect, DropTargetPos::Above);
            if upper_hovered {
                self.hovered = Some(DropAction::Before {
                    parent_id,
                    child_id: *self.node.get_id(),
                });
                ui.painter().set(where_to_put_background, shape);
            }

            let (lower_hovered, shape) =
                Self::check_drop_target(ui, row_rect, response.rect, DropTargetPos::Below);
            if lower_hovered {
                self.hovered = Some(DropAction::After {
                    parent_id,
                    child_id: *self.node.get_id(),
                });
                ui.painter().set(where_to_put_background, shape);
            }
        }

        let (middle_hover, shape) =
            Self::check_drop_target(ui, row_rect, response.rect, DropTargetPos::On);
        if middle_hover {
            self.hovered = Some(DropAction::On {
                parent_id: *self.node.get_id(),
            });
            ui.painter().set(where_to_put_background, shape);
        }

        result
    }

    fn leaf_drop_target<T>(
        &mut self,
        ui: &mut Ui,
        mut add_content: impl FnMut(&mut Self, &mut Ui) -> (Rect, T),
    ) -> T {
        let where_to_put_background = ui.painter().add(Shape::Noop);
        // let where_to_put_upper = ui.painter().add(Shape::Noop);
        // let where_to_put_lower = ui.painter().add(Shape::Noop);

        let InnerResponse {
            inner: (row_rect, result),
            response,
        } = ui.allocate_ui(ui.available_size_before_wrap(), |ui| add_content(self, ui));

        if self
            .was_dragged_last_frame
            .map_or(true, |id| &id == self.node.get_id())
        {
            return result;
        }

        if let Some(parent_id) = self.parent {
            let (upper_hovered, shape) =
                Self::check_drop_target(ui, row_rect, response.rect, DropTargetPos::Upper);
            if upper_hovered {
                self.hovered = Some(DropAction::Before {
                    parent_id,
                    child_id: *self.node.get_id(),
                });
                ui.painter().set(where_to_put_background, shape);
            }

            let (lower_hovered, shape) =
                Self::check_drop_target(ui, row_rect, response.rect, DropTargetPos::Lower);
            if lower_hovered {
                self.hovered = Some(DropAction::After {
                    parent_id,
                    child_id: *self.node.get_id(),
                });
                ui.painter().set(where_to_put_background, shape);
            }
        }

        result
    }

    fn drag_source<T>(
        &mut self,
        ui: &mut Ui,
        mut add_content: impl FnMut(&mut Self, &mut Ui) -> ShowNodeResult<T>,
    ) -> ShowNodeResult<T> {
        let rect = Rect::from_min_size(ui.cursor().min, Vec2::ZERO);
        let drag_source_id = ui.next_auto_id().with("Drag source id");
        ui.ctx()
            .check_for_id_clash(drag_source_id, rect, "Drag source");

        let (is_dragged, drag_offset) = ui
            .data_mut(|d| d.get_persisted::<(bool, Vec2)>(drag_source_id))
            .unwrap_or((false, Vec2::ZERO));

        let result = if is_dragged {
            self.dragged = Some(*self.node.get_id());

            ui.ctx().set_cursor_icon(egui::CursorIcon::Grabbing);

            let result = add_content(self, ui);

            // Paint the content again to a new layer for the drag overlay.
            let layer_id = LayerId::new(Order::Tooltip, drag_source_id);
            let response = ui
                .child_ui(ui.available_rect_before_wrap(), *ui.layout())
                .with_layer_id(layer_id, |ui| self.drag_overlay_background(ui, add_content));

            // Move layer to the drag position
            if let Some(pointer_pos) = ui.ctx().pointer_interact_pos() {
                let delta = pointer_pos - response.inner.rect.min + drag_offset;
                ui.ctx().translate_layer(layer_id, delta);
            }

            result
        } else {
            add_content(self, ui)
        };

        // Save the drag offset and drag value for next frame.
        let drag_offset = (!is_dragged && result.dragged)
            .then_some(())
            .and_then(|_| ui.ctx().pointer_interact_pos())
            .map(|pointer_pos| result.rect.min - pointer_pos)
            .unwrap_or(drag_offset);
        ui.data_mut(|d| d.insert_persisted(drag_source_id, (result.dragged, drag_offset)));
        result
    }

    fn show_dir_header(&mut self, ui: &mut Ui) -> ShowNodeResult<SplitCollapsingState<()>> {
        // Generate id out of the node id to make sure that the header in the tree
        // and the drag overlay are the same.
        let collapsing_id = Id::new(self.node.get_id()).with("dir header");

        let mut state = SplitCollapsingState::show_header(ui, collapsing_id, |ui| {
            self.node.show(ui);
            ui.allocate_at_least(vec2(ui.available_width(), 0.0), Sense::hover());
        });

        let (left, right) = {
            let SplitCollapsingState {
                button_response: button,
                header_response: header,
                ..
            } = &state;

            let right_of_button = ui.interact(
                Rect::from_min_max(
                    pos2(button.rect.right(), header.response.rect.top()),
                    pos2(self.bounds.right(), header.response.rect.bottom()),
                ),
                collapsing_id.with(1),
                Sense::click_and_drag(),
            );
            let left_of_button = ui.interact(
                Rect::from_min_max(
                    pos2(self.bounds.left(), header.response.rect.top()),
                    pos2(button.rect.left(), header.response.rect.bottom()),
                ),
                collapsing_id.with(2),
                Sense::click_and_drag(),
            );
            (left_of_button, right_of_button)
        };
        if right.double_clicked() || left.double_clicked() {
            state.toggle(ui);
        }

        ShowNodeResult {
            rect: Rect::from_min_max(left.rect.min, right.rect.max)
                .expand2(vec2(0.0, ui.spacing().item_spacing.y / 2.0)),
            clicked: right.clicked() || left.clicked(),
            dragged: right.dragged() || left.dragged(),
            inner: state,
        }
    }
    fn show_dir_body(&mut self, ui: &mut Ui, state: &SplitCollapsingState<()>) {
        state.show_body(ui, |ui| {
            for child in self.node.get_children() {
                let mut c = Self {
                    bounds: self.bounds,
                    line_count: self.line_count,
                    was_dragged_last_frame: self.was_dragged_last_frame,
                    dragged: self.dragged,
                    hovered: self.hovered.clone(),
                    node: child,
                    selected: self.selected,
                    parent: Some(*self.node.get_id()),
                };

                c.show_node(ui);

                self.line_count = c.line_count;
                self.dragged = c.dragged;
                self.hovered = c.hovered;
                self.selected = c.selected;
            }
        });
    }

    fn show_leaf(&mut self, ui: &mut Ui) -> ShowNodeResult<()> {
        let res = ui.scope(|ui| {
            ui.horizontal(|ui| {
                self.node.show(ui);
                ui.allocate_at_least(vec2(ui.available_width(), 0.0), Sense::hover());
            });
        });

        let full_width = Rect::from_min_max(
            pos2(self.bounds.left(), res.response.rect.top()),
            pos2(self.bounds.right(), res.response.rect.bottom()),
        )
        .expand2(vec2(0.0, ui.spacing().item_spacing.y / 2.0));

        let full_width_res =
            ui.interact(full_width, res.response.id.with(1), Sense::click_and_drag());
        ShowNodeResult {
            rect: full_width,
            clicked: full_width_res.clicked(),
            dragged: full_width_res.dragged(),
            inner: (),
        }
    }

    fn drag_overlay_background<T>(
        &mut self,
        ui: &mut Ui,
        mut add_content: impl FnMut(&mut Self, &mut Ui) -> ShowNodeResult<T>,
    ) -> ShowNodeResult<T> {
        let background = ui.painter().add(Shape::Noop);
        let result = add_content(self, ui);
        ui.painter().set(
            background,
            epaint::RectShape {
                rect: result.rect,
                rounding: ui.visuals().widgets.active.rounding,
                fill: ui.visuals().selection.bg_fill.linear_multiply(0.5),
                stroke: Stroke::NONE,
            },
        );
        result
    }
}
