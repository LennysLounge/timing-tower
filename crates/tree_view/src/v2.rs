use bevy_egui::egui::{
    epaint, vec2, Color32, CursorIcon, Id, LayerId, Layout, Order, PointerButton, Rect, Response,
    Rounding, Sense, Shape, Stroke, Ui, Vec2,
};
use uuid::Uuid;

#[derive(Clone)]
struct TreeViewBuilderState {
    visible: bool,
}
pub struct TreeViewBuilder2<'a> {
    ui: &'a mut Ui,
    selected: &'a mut Option<Uuid>,
    state: TreeViewBuilderState,
    stack: Vec<TreeViewBuilderState>,
}

impl<'a> TreeViewBuilder2<'a> {
    pub fn new(ui: &mut Ui, base_id: Id, mut add_content: impl FnMut(TreeViewBuilder2<'_>)) {
        let mut selected = ui
            .data_mut(|d| d.get_persisted::<Option<Uuid>>(base_id))
            .flatten();

        let mut child_ui = ui.child_ui_with_id_source(
            Rect::from_min_size(ui.cursor().min, vec2(200., ui.available_height())),
            Layout::top_down(bevy_egui::egui::Align::Min),
            base_id,
        );

        let rect = {
            child_ui.spacing_mut().item_spacing.y = 20.0;

            child_ui.add_space(child_ui.spacing().item_spacing.y * 0.5);
            add_content(TreeViewBuilder2 {
                ui: &mut child_ui,
                selected: &mut selected,
                state: TreeViewBuilderState { visible: true },
                stack: Vec::new(),
            });
            // Add negative space because the place will add the item spacing on top of this.
            child_ui.add_space(-child_ui.spacing().item_spacing.y * 0.5);
            child_ui.min_rect()
        };

        ui.painter()
            .rect_stroke(rect, Rounding::ZERO, Stroke::new(1.0, Color32::BLACK));

        ui.data_mut(|d| d.insert_persisted(base_id, selected));
    }

    pub fn leaf(&mut self, id: &Uuid, mut add_content: impl FnMut(&mut Ui)) {
        if !self.state.visible {
            return;
        }

        let row_id = self.ui.id().with(id);
        let row_rect = self
            .ui
            .data_mut(|d| d.get_persisted::<Rect>(row_id))
            .unwrap_or(Rect::NOTHING);

        let interaction = {
            let spacing_before = self.ui.spacing().clone();
            self.ui.spacing_mut().item_spacing = Vec2::ZERO;
            let res = self.ui.interact(row_rect, row_id, Sense::click_and_drag());
            *self.ui.spacing_mut() = spacing_before;
            res
        };

        if interaction.drag_started() {
            *self.selected = Some(*id);
            println!("{} was clicked with egui_id: {:?}", id, row_id);
        }
        if interaction.dragged_by(PointerButton::Primary)
            || interaction.drag_released_by(PointerButton::Primary)
        {
            self.draw_drag_overlay(&interaction, &mut add_content);
        }

        let background_position = self.ui.painter().add(Shape::Noop);

        let res = self
            .ui
            .horizontal(|ui| {
                ui.allocate_response(vec2(20.0 * self.stack.len() as f32, 0.0), Sense::hover());
                add_content(ui);
                ui.allocate_response(vec2(ui.available_width(), 0.0), Sense::hover());
            })
            .response;

        let background_rect = res
            .rect
            .expand2(vec2(0.0, self.ui.spacing().item_spacing.y * 0.5));

        if self
            .selected
            .as_ref()
            .is_some_and(|selected_id| selected_id == id)
        {
            self.ui.painter().set(
                background_position,
                epaint::RectShape::new(
                    background_rect,
                    Rounding::ZERO,
                    Color32::RED.linear_multiply(0.4),
                    Stroke::NONE,
                ),
            );
        } else {
            self.ui.painter().set(
                background_position,
                epaint::RectShape::new(
                    background_rect,
                    Rounding::ZERO,
                    Color32::RED.linear_multiply(0.2),
                    Stroke::NONE,
                ),
            );
        }

        self.ui
            .data_mut(|d| d.insert_persisted(row_id, background_rect));
    }

    pub fn dir(&mut self, id: &Uuid, mut add_content: impl FnMut(&mut Ui)) {
        #[derive(Clone)]
        struct DirState {
            rect: Rect,
            visible: bool,
        }

        if !self.state.visible {
            self.stack.push(self.state.clone());
            return;
        }

        let dir_id = self.ui.id().with(id);
        let mut dir_state = self
            .ui
            .data_mut(|d| d.get_persisted::<DirState>(dir_id))
            .unwrap_or(DirState {
                rect: Rect::NOTHING,
                visible: true,
            });

        let res = {
            let spacing_before = self.ui.spacing().clone();
            self.ui.spacing_mut().item_spacing = Vec2::ZERO;
            let res = self
                .ui
                .interact(dir_state.rect, dir_id, Sense::click_and_drag());
            *self.ui.spacing_mut() = spacing_before;
            res
        };

        if res.drag_started() {
            *self.selected = Some(*id);
            dir_state.visible = !dir_state.visible;
            println!("{} was clicked", id);
        }

        let background_position = self.ui.painter().add(Shape::Noop);

        let res = self
            .ui
            .horizontal(|ui| {
                ui.allocate_response(vec2(20.0 * self.stack.len() as f32, 0.0), Sense::hover());
                add_content(ui);
                ui.allocate_response(vec2(ui.available_width(), 0.0), Sense::hover());
            })
            .response;

        dir_state.rect = res
            .rect
            .expand2(vec2(0.0, self.ui.spacing().item_spacing.y * 0.5));

        if self
            .selected
            .as_ref()
            .is_some_and(|selected_id| selected_id == id)
        {
            self.ui.painter().set(
                background_position,
                epaint::RectShape::new(
                    dir_state.rect,
                    Rounding::ZERO,
                    Color32::BLUE.linear_multiply(0.4),
                    Stroke::NONE,
                ),
            );
        } else {
            self.ui.painter().set(
                background_position,
                epaint::RectShape::new(
                    dir_state.rect,
                    Rounding::ZERO,
                    Color32::BLUE.linear_multiply(0.2),
                    Stroke::NONE,
                ),
            );
        }

        self.ui
            .data_mut(|d| d.insert_persisted(dir_id, dir_state.clone()));

        self.stack.push(self.state.clone());
        self.state = TreeViewBuilderState {
            visible: dir_state.visible,
        }
    }

    pub fn close_dir(&mut self) {
        self.state = self.stack.pop().expect("Stack was empty");
    }

    /// Draw the content as a drag overlay if it is beeing dragged.
    fn draw_drag_overlay(&mut self, interaction: &Response, add_content: &mut impl FnMut(&mut Ui)) {
        let drag_source_id = self.ui.make_persistent_id("Drag source");
        let drag_offset = if interaction.drag_started_by(PointerButton::Primary) {
            let drag_offset = self
                .ui
                .ctx()
                .pointer_latest_pos()
                .map(|pointer_pos| interaction.rect.min - pointer_pos)
                .unwrap_or(Vec2::ZERO);
            self.ui
                .data_mut(|d| d.insert_persisted::<Vec2>(drag_source_id, drag_offset));
            drag_offset
        } else {
            self.ui
                .data_mut(|d| d.get_persisted::<Vec2>(drag_source_id))
                .unwrap_or(Vec2::ZERO)
        };

        //context.dragged = Some(self.id);
        self.ui.ctx().set_cursor_icon(CursorIcon::Alias);

        // Paint the content to a new layer for the drag overlay.
        let layer_id = LayerId::new(Order::Tooltip, drag_source_id);
        let background_rect = self
            .ui
            .child_ui(self.ui.available_rect_before_wrap(), *self.ui.layout())
            .with_layer_id(layer_id, |ui| {
                let background = ui.painter().add(Shape::Noop);

                let res = ui.horizontal(|ui| {
                    ui.allocate_response(vec2(20.0 * self.stack.len() as f32, 0.0), Sense::hover());
                    add_content(ui);
                    ui.allocate_response(vec2(ui.available_width(), 0.0), Sense::hover());
                });
                let rect = res
                    .response
                    .rect
                    .expand2(vec2(0.0, ui.spacing().item_spacing.y * 0.5));

                ui.painter().set(
                    background,
                    epaint::RectShape::new(
                        rect,
                        ui.visuals().widgets.active.rounding,
                        ui.visuals().selection.bg_fill.linear_multiply(0.5),
                        Stroke::NONE,
                    ),
                );
                res.response.with_new_rect(rect)
            })
            .inner
            .rect;

        // Move layer to the drag position
        if let Some(pointer_pos) = self.ui.ctx().pointer_interact_pos() {
            let delta = pointer_pos - background_rect.min + drag_offset;
            self.ui.ctx().translate_layer(layer_id, delta);
        }
    }
}
