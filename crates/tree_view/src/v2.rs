use bevy_egui::egui::{
    epaint, vec2, Color32, CursorIcon, Id, InnerResponse, LayerId, Layout, Order, PointerButton,
    Pos2, Rangef, Rect, Response, Rounding, Sense, Shape, Stroke, Ui, Vec2,
};
use uuid::Uuid;

use crate::DropPosition;

#[derive(Clone)]
struct DirectoryState {
    /// If directory is expanded
    is_open: bool,
    /// Id of the directory node.
    id: Option<Uuid>,
    /// Counter to keep track of directories that are hidden inside this dir.
    invisible_dirs_stack: i32,
}
pub struct TreeViewBuilder2<'a> {
    ui: &'a mut Ui,
    selected: &'a mut Option<Uuid>,
    drag: &'a mut Option<Uuid>,
    drop: &'a mut Option<(Uuid, DropPosition)>,
    current_dir: DirectoryState,
    stack: Vec<DirectoryState>,
}

impl<'a> TreeViewBuilder2<'a> {
    pub fn new(ui: &mut Ui, base_id: Id, mut add_content: impl FnMut(TreeViewBuilder2<'_>)) {
        let mut selected = ui
            .data_mut(|d| d.get_persisted::<Option<Uuid>>(base_id))
            .flatten();
        let mut drag = None;
        let mut drop = None;

        let mut child_ui = ui.child_ui_with_id_source(
            Rect::from_min_size(ui.cursor().min, vec2(200., ui.available_height())),
            Layout::top_down(bevy_egui::egui::Align::Min),
            base_id,
        );

        let rect = {
            child_ui.spacing_mut().item_spacing.y = 15.0;

            child_ui.add_space(child_ui.spacing().item_spacing.y * 0.5);
            add_content(TreeViewBuilder2 {
                ui: &mut child_ui,
                selected: &mut selected,
                drag: &mut drag,
                drop: &mut drop,
                current_dir: DirectoryState {
                    is_open: true,
                    id: None,
                    invisible_dirs_stack: 0,
                },
                stack: Vec::new(),
            });
            // Add negative space because the place will add the item spacing on top of this.
            child_ui.add_space(-child_ui.spacing().item_spacing.y * 0.5);
            child_ui.min_rect()
        };
        ui.allocate_rect(child_ui.min_rect(), Sense::hover());

        ui.painter()
            .rect_stroke(rect, Rounding::ZERO, Stroke::new(1.0, Color32::BLACK));
        ui.data_mut(|d| d.insert_persisted(base_id, selected));

        ui.label(format!("drag: {:?}", drag));
        ui.label(format!("drop: {:?}", drop));
    }

    fn row(
        &mut self,
        node_config: &NodeConfig,
        mut add_content: impl FnMut(&mut Ui),
    ) -> InnerResponse<Response> {
        // Load row data
        let row_id = self.ui.id().with(node_config.id).with("row");
        let row_rect = self
            .ui
            .data_mut(|d| d.get_persisted::<Rect>(row_id))
            .unwrap_or(Rect::NOTHING);

        // Interact with the row
        let interaction = self.interact(row_rect, row_id, Sense::click_and_drag());

        if interaction.drag_started() {
            *self.selected = Some(node_config.id);
            println!("{} was clicked with egui_id: {:?}", node_config.id, row_id);
        }

        self.drag(&interaction, &node_config.id, &mut add_content);
        self.drop(&interaction, node_config);

        let row_response = self.draw_row(add_content, self.is_selected(&node_config.id));

        // Store row data
        self.ui
            .data_mut(|d| d.insert_persisted(row_id, row_response.rect));

        InnerResponse::new(interaction, row_response)
    }

    pub fn leaf(&mut self, id: &Uuid, add_content: impl FnMut(&mut Ui)) {
        if !self.current_dir.is_open {
            return;
        }
        let node_config = NodeConfig {
            parent: self.current_dir.id,
            id: *id,
            drop_on_allowed: false,
        };
        self.row(&node_config, add_content);
    }

    pub fn dir(&mut self, id: &Uuid, add_content: impl FnMut(&mut Ui)) {
        if !self.current_dir.is_open {
            self.current_dir.invisible_dirs_stack += 1;
            return;
        }

        let node_config = NodeConfig {
            parent: self.current_dir.id,
            id: *id,
            drop_on_allowed: true,
        };

        let InnerResponse {
            inner: interaction,
            response: _,
        } = self.row(&node_config, add_content);

        let dir_id = self.ui.id().with(id).with("dir");
        let mut open = self
            .ui
            .data_mut(|d| d.get_persisted(dir_id))
            .unwrap_or(true);

        if interaction.double_clicked() {
            open = !open;
        }

        self.ui.data_mut(|d| d.insert_persisted(dir_id, open));

        self.stack.push(self.current_dir.clone());
        self.current_dir = DirectoryState {
            is_open: open,
            id: Some(*id),
            invisible_dirs_stack: 0,
        }
    }

    pub fn close_dir(&mut self) {
        if self.current_dir.invisible_dirs_stack > 0 {
            self.current_dir.invisible_dirs_stack -= 1;
        } else {
            self.current_dir = self.stack.pop().expect("Stack was empty");
        }
    }

    /// Draw the content as a drag overlay if it is beeing dragged.
    fn drag(&mut self, interaction: &Response, id: &Uuid, add_content: &mut impl FnMut(&mut Ui)) {
        if !interaction.dragged_by(PointerButton::Primary)
            && !interaction.drag_released_by(PointerButton::Primary)
        {
            return;
        }

        *self.drag = Some(*id);
        self.ui.ctx().set_cursor_icon(CursorIcon::Alias);

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

    fn drop(&mut self, interaction: &Response, node_config: &NodeConfig) {
        pub const DROP_LINE_HEIGHT: f32 = 3.0;

        if !self.ui.ctx().memory(|m| m.is_anything_being_dragged()) {
            return;
        }
        if self.is_selected(&node_config.id) {
            return;
        }

        // For some reason we cannot use the provided interation response
        // because once a row is dragged all other rows dont offer any hover information.
        // To fix this we interaction with only hover again.
        let cursor_y = {
            let Some(Pos2 { y, .. }) = self
                .interact(
                    interaction.rect,
                    self.ui.make_persistent_id("Drop target"),
                    Sense::hover(),
                )
                .hover_pos()
            else {
                return;
            };
            y
        };

        let Some(drop_quater) = DropQuater::new(interaction.rect.y_range(), cursor_y) else {
            return;
        };

        if let Some((parent_id, drop_position)) = drop_quater.get_drop_position(node_config) {
            let drop_marker = match &drop_position {
                DropPosition::Before(_) => {
                    Rangef::point(interaction.rect.min.y).expand(DROP_LINE_HEIGHT * 0.5)
                }
                DropPosition::First | DropPosition::After(_) => {
                    Rangef::point(interaction.rect.max.y).expand(DROP_LINE_HEIGHT * 0.5)
                }
                DropPosition::Last => interaction.rect.y_range(),
            };

            self.ui.painter().add(epaint::RectShape::new(
                Rect::from_x_y_ranges(interaction.rect.x_range(), drop_marker),
                self.ui.visuals().widgets.active.rounding,
                self.ui.style().visuals.selection.bg_fill,
                Stroke::NONE,
            ));

            *self.drop = Some((parent_id, drop_position));
        }
    }

    fn draw_row(
        &mut self,
        mut add_content: impl FnMut(&mut Ui),
        draw_background: bool,
    ) -> Response {
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

        if draw_background {
            self.ui.painter().set(
                background_position,
                epaint::RectShape::new(
                    background_rect,
                    Rounding::ZERO,
                    self.ui.visuals().selection.bg_fill,
                    Stroke::NONE,
                ),
            );
        }
        res.with_new_rect(background_rect)
    }

    /// Interact with the ui without egui adding any extra space.
    fn interact(&mut self, rect: Rect, id: Id, sense: Sense) -> Response {
        let spacing_before = self.ui.spacing().clone();
        self.ui.spacing_mut().item_spacing = Vec2::ZERO;
        let res = self.ui.interact(rect, id, sense);
        *self.ui.spacing_mut() = spacing_before;
        res
    }

    fn is_selected(&self, id: &Uuid) -> bool {
        self.selected
            .as_ref()
            .is_some_and(|selected_id| selected_id == id)
    }
}

struct NodeConfig {
    parent: Option<Uuid>,
    id: Uuid,
    drop_on_allowed: bool,
}

enum DropQuater {
    Top,
    MiddleTop,
    MiddleBottom,
    Bottom,
}

impl DropQuater {
    fn new(range: Rangef, cursor_pos: f32) -> Option<DropQuater> {
        pub const DROP_LINE_HOVER_HEIGHT: f32 = 5.0;

        let h0 = range.min;
        let h1 = range.min + DROP_LINE_HOVER_HEIGHT;
        let h2 = (range.min + range.max) / 2.0;
        let h3 = range.max - DROP_LINE_HOVER_HEIGHT;
        let h4 = range.max;

        match cursor_pos {
            y if y >= h0 && y < h1 => Some(Self::Top),
            y if y >= h1 && y < h2 => Some(Self::MiddleTop),
            y if y >= h2 && y < h3 => Some(Self::MiddleBottom),
            y if y >= h3 && y < h4 => Some(Self::Bottom),
            _ => None,
        }
    }
    fn get_drop_position(&self, node_config: &NodeConfig) -> Option<(Uuid, DropPosition)> {
        let NodeConfig {
            parent,
            id,
            drop_on_allowed,
        } = node_config;

        match self {
            DropQuater::Top => {
                if let Some(parent_id) = parent {
                    return Some((*parent_id, DropPosition::Before(*id)));
                }
                if *drop_on_allowed {
                    return Some((*id, DropPosition::Last));
                }
                return None;
            }
            DropQuater::MiddleTop => {
                if *drop_on_allowed {
                    return Some((*id, DropPosition::Last));
                }
                if let Some(parent_id) = parent {
                    return Some((*parent_id, DropPosition::Before(*id)));
                }
                return None;
            }
            DropQuater::MiddleBottom => {
                if *drop_on_allowed {
                    return Some((*id, DropPosition::Last));
                }
                if let Some(parent_id) = parent {
                    return Some((*parent_id, DropPosition::After(*id)));
                }
                return None;
            }
            DropQuater::Bottom => {
                if let Some(parent_id) = parent {
                    return Some((*parent_id, DropPosition::After(*id)));
                }
                if *drop_on_allowed {
                    return Some((*id, DropPosition::Last));
                }
                return None;
            }
        }
    }
}
