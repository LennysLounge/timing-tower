use bevy_egui::egui::{
    self, collapsing_header::CollapsingState, epaint, pos2, vec2, Color32, LayerId, Order, Rect,
    Rounding, Sense, Shape, Stroke, Ui, Vec2,
};
use uuid::Uuid;

pub trait TreeNode {
    fn is_directory(&self) -> bool;
    fn show(&self, ui: &mut Ui);
    fn get_children(&self) -> Vec<&dyn TreeNode>;
    fn get_id(&self) -> &Uuid;
}

pub struct TreeView {
    pub selected: Option<Uuid>,
}

impl TreeView {
    pub fn show(&mut self, ui: &mut Ui, root: &impl TreeNode) {
        let res = ui.allocate_ui(ui.available_size_before_wrap(), |ui| {
            let mut context = TreeViewContext {
                tree_view: self,
                bounds: ui.available_rect_before_wrap(),
                line_count: 0,
            };
            context.show_node(ui, root);
        });

        ui.painter().rect_stroke(
            res.response.rect,
            Rounding::none(),
            Stroke::new(1.0, Color32::BLACK),
        );
    }
}

struct TreeViewContext<'a> {
    tree_view: &'a mut TreeView,
    bounds: Rect,
    line_count: i32,
}

struct ShowNodeResult {
    rect: Rect,
    clicked: bool,
    dragged: bool,
}

impl<'a> TreeViewContext<'a> {
    fn show_node(&mut self, ui: &mut Ui, node: &dyn TreeNode) {
        let mut child_ui = ui.child_ui_with_id_source(
            ui.available_rect_before_wrap(),
            *ui.layout(),
            node.get_id(),
        );
        self.show_node_ui(&mut child_ui, node);
        ui.allocate_at_least(child_ui.min_rect().size(), Sense::hover());
    }

    fn show_node_ui(&mut self, ui: &mut Ui, node: &dyn TreeNode) {
        let where_to_put_background = ui.painter().add(Shape::Noop);

        self.line_count += 1;
        let is_even = self.line_count % 2 == 0;
        let is_selected = self
            .tree_view
            .selected
            .is_some_and(|id| &id == node.get_id());

        let node_result = if node.is_directory() {
            self.drag_source(ui, |me, ui| me.show_dir(ui, node))
        } else {
            self.drag_source(ui, |me, ui| me.show_leaf(ui, node))
        };

        if node_result.clicked || node_result.dragged {
            self.tree_view.selected = Some(*node.get_id());
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

    fn _drop_target<T>(
        &mut self,
        ui: &mut Ui,
        mut add_content: impl FnMut(&mut Self, &mut Ui) -> T,
    ) -> T {
        let where_to_put_background = ui.painter().add(Shape::Noop);

        let mut content_ui = ui.child_ui(ui.available_rect_before_wrap(), *ui.layout());

        let res = add_content(self, &mut content_ui);

        let (rect, response) = ui.allocate_at_least(content_ui.min_rect().size(), Sense::hover());
        let is_hovered = response.hovered();

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
                fill: Color32::TRANSPARENT,
                stroke: style.bg_stroke,
            },
        );
        res
    }

    fn drag_source(
        &mut self,
        ui: &mut Ui,
        mut add_content: impl FnMut(&mut Self, &mut Ui) -> ShowNodeResult,
    ) -> ShowNodeResult {
        let rect = Rect::from_min_size(ui.cursor().min, Vec2::ZERO);
        let drag_source_id = ui.next_auto_id().with("Drag source id");
        ui.ctx()
            .check_for_id_clash(drag_source_id, rect, "Drag source");

        let (is_dragged, drag_offset) = ui
            .data_mut(|d| d.get_persisted::<(bool, Vec2)>(drag_source_id))
            .unwrap_or((false, Vec2::ZERO));

        let result = if is_dragged {
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

    fn show_dir(&mut self, ui: &mut Ui, node: &dyn TreeNode) -> ShowNodeResult {
        let collapsing_id = ui.next_auto_id();
        let (button, header, _) =
            CollapsingState::load_with_default_open(ui.ctx(), collapsing_id, true)
                .show_header(ui, |ui| {
                    node.show(ui);
                    ui.allocate_at_least(vec2(ui.available_width(), 0.0), Sense::hover());
                })
                .body(|ui| {
                    for child in node.get_children() {
                        self.show_node(ui, child);
                    }
                });

        let left_of_button = Rect::from_min_max(
            pos2(self.bounds.left(), header.response.rect.top()),
            pos2(button.rect.left(), header.response.rect.bottom()),
        );

        let right_of_button = Rect::from_min_max(
            pos2(button.rect.right(), header.response.rect.top()),
            pos2(self.bounds.right(), header.response.rect.bottom()),
        );

        let right_of_button = ui.interact(
            right_of_button,
            collapsing_id.with(1),
            Sense::click_and_drag(),
        );
        let left_of_button = ui.interact(
            left_of_button,
            collapsing_id.with(2),
            Sense::click_and_drag(),
        );
        if right_of_button.double_clicked() || left_of_button.double_clicked() {
            if let Some(mut state) = CollapsingState::load(ui.ctx(), collapsing_id) {
                state.toggle(ui);
                state.store(ui.ctx());
            }
        }

        ShowNodeResult {
            rect: Rect::from_min_max(
                pos2(self.bounds.left(), header.response.rect.top()),
                pos2(self.bounds.right(), header.response.rect.bottom()),
            ),
            clicked: right_of_button.clicked() || left_of_button.clicked(),
            dragged: right_of_button.dragged() || left_of_button.dragged(),
        }
    }

    fn show_leaf(&mut self, ui: &mut Ui, node: &dyn TreeNode) -> ShowNodeResult {
        let res = ui.scope(|ui| {
            ui.horizontal(|ui| {
                node.show(ui);
                ui.allocate_at_least(vec2(ui.available_width(), 0.0), Sense::hover());
            });
        });
        let full_width = Rect::from_min_max(
            pos2(self.bounds.left(), res.response.rect.top()),
            pos2(self.bounds.right(), res.response.rect.bottom()),
        );

        let full_width_res =
            ui.interact(full_width, res.response.id.with(1), Sense::click_and_drag());
        ShowNodeResult {
            rect: full_width,
            clicked: full_width_res.clicked(),
            dragged: full_width_res.dragged(),
        }
    }

    fn drag_overlay_background(
        &mut self,
        ui: &mut Ui,
        mut add_content: impl FnMut(&mut Self, &mut Ui) -> ShowNodeResult,
    ) -> ShowNodeResult {
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
