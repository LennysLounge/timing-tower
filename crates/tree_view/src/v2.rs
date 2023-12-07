use bevy_egui::egui::{
    self,
    epaint::{self, RectShape},
    layers::ShapeIdx,
    vec2, Color32, CursorIcon, Id, InnerResponse, LayerId, Layout, Order, PointerButton, Pos2,
    Rangef, Rect, Response, Rounding, Sense, Shape, Stroke, Ui, Vec2,
};
use uuid::Uuid;

use crate::DropPosition;

pub enum TreeViewAction {
    Drop {
        node_to_remove: Uuid,
        receiver_node: Uuid,
        position: DropPosition,
    },
}

#[derive(Clone)]
struct DirectoryState {
    /// Id of the directory node.
    id: Uuid,
    /// If directory is expanded
    is_open: bool,
    /// If a directory is dragged, dropping is disallowed for any of
    /// its child nodes.
    drop_forbidden: bool,
    /// The rectangle of the row.
    row_rect: Rect,
    /// The rectangle of the icon.
    icon_rect: Rect,
    /// The shape index where the drop marker is drawn.
    drop_marker_idx: ShapeIdx,
}
struct NodeConfig<'a> {
    id: Uuid,
    drop_on_allowed: bool,
    is_open: bool,
    add_content: &'a mut dyn FnMut(&mut Ui),
    add_icon: Option<&'a mut dyn FnMut(&mut Ui, Rect) -> Response>,
    drop_marker_idx: ShapeIdx,
}
pub struct TreeViewBuilder2<'a> {
    ui: &'a mut Ui,
    selected: &'a mut Option<Uuid>,
    drag: &'a mut Option<Uuid>,
    drop: &'a mut Option<(Uuid, DropPosition)>,
    stack: Vec<DirectoryState>,
    was_dragged_last_frame: bool,
}

impl<'a> TreeViewBuilder2<'a> {
    pub fn new(
        ui: &mut Ui,
        base_id: Id,
        mut add_content: impl FnMut(TreeViewBuilder2<'_>),
    ) -> InnerResponse<Vec<TreeViewAction>> {
        #[derive(Clone, Default)]
        struct TreeViewBuilderState {
            selected: Option<Uuid>,
            was_dragged_last_frame: bool,
        }
        let mut state = ui
            .data_mut(|d| d.get_persisted::<TreeViewBuilderState>(base_id))
            .unwrap_or_default();

        let mut drag = None;
        let mut drop = None;

        let mut child_ui = ui.child_ui_with_id_source(
            Rect::from_min_size(ui.cursor().min, vec2(200., ui.available_height())),
            Layout::top_down(bevy_egui::egui::Align::Min),
            base_id,
        );

        let rect = {
            child_ui.spacing_mut().item_spacing.y = 7.0;
            child_ui.spacing_mut().indent = 20.0;

            child_ui.add_space(child_ui.spacing().item_spacing.y * 0.5);
            add_content(TreeViewBuilder2 {
                ui: &mut child_ui,
                selected: &mut state.selected,
                drag: &mut drag,
                drop: &mut drop,
                stack: Vec::new(),
                was_dragged_last_frame: state.was_dragged_last_frame,
            });
            // Add negative space because the place will add the item spacing on top of this.
            child_ui.add_space(-child_ui.spacing().item_spacing.y * 0.5);
            child_ui.min_rect()
        };
        let res = ui.allocate_rect(child_ui.min_rect(), Sense::hover());

        ui.painter()
            .rect_stroke(rect, Rounding::ZERO, Stroke::new(1.0, Color32::BLACK));

        state.was_dragged_last_frame = drag.is_some();
        ui.data_mut(|d| d.insert_persisted(base_id, state));

        ui.label(format!("drag: {:?}", drag));
        ui.label(format!("drop: {:?}", drop));

        let mut actions = Vec::new();
        if ui.ctx().input(|i| i.pointer.any_released()) {
            if let (Some(node_to_remove), Some((receiver_node, position))) = (drag, drop) {
                actions.push(TreeViewAction::Drop {
                    node_to_remove,
                    receiver_node,
                    position,
                });
            }
        }

        InnerResponse::new(actions, res)
    }

    pub fn leaf(&mut self, id: &Uuid, mut add_content: impl FnMut(&mut Ui)) -> Option<Response> {
        if !self.current_dir_is_open() {
            return None;
        }
        let mut node_config = NodeConfig {
            id: *id,
            drop_on_allowed: false,
            is_open: false,
            add_content: &mut add_content,
            add_icon: None,
            drop_marker_idx: self.ui.painter().add(Shape::Noop),
        };
        Some(self.row(&mut node_config).interaction)
    }

    pub fn dir(&mut self, id: &Uuid, mut add_content: impl FnMut(&mut Ui)) -> Option<Response> {
        if !self.current_dir_is_open() {
            self.stack.push(DirectoryState {
                is_open: false,
                id: *id,
                drop_forbidden: true,
                row_rect: Rect::NOTHING,
                icon_rect: Rect::NOTHING,
                drop_marker_idx: self.ui.painter().add(Shape::Noop),
            });
            return None;
        }

        let dir_id = self.ui.id().with(id).with("dir");
        let mut open = self
            .ui
            .data_mut(|d| d.get_persisted(dir_id))
            .unwrap_or(true);

        let mut add_icon = |ui: &mut Ui, rect| {
            let icon_res = ui.allocate_rect(rect, Sense::click());
            let openness = ui.ctx().animate_bool(icon_res.id, open);
            egui::collapsing_header::paint_default_icon(ui, openness, &icon_res);
            icon_res
        };

        let mut node_config = NodeConfig {
            id: *id,
            drop_on_allowed: true,
            is_open: open,
            add_content: &mut add_content,
            add_icon: Some(&mut add_icon),
            drop_marker_idx: self.ui.painter().add(Shape::Noop),
        };

        let RowResponse {
            interaction,
            visual,
            icon,
        } = self.row(&mut node_config);
        let drop_marker_idx = node_config.drop_marker_idx;

        if interaction.double_clicked() {
            open = !open;
        }

        let icon = icon.expect("Icon response is not available");
        if icon.clicked() {
            open = !open;
            *self.selected = Some(*id);
        }

        self.ui.data_mut(|d| d.insert_persisted(dir_id, open));

        //self.stack.push(self.current_dir.clone());
        self.stack.push(DirectoryState {
            is_open: open,
            id: *id,
            drop_forbidden: self.current_dir_drop_forbidden() || self.is_dragged(id),
            row_rect: visual.rect,
            icon_rect: icon.rect,
            drop_marker_idx,
        });
        Some(interaction)
    }

    pub fn close_dir(&mut self) {
        if let Some(current_dir) = self.current_dir() {
            if let Some((drop_parent, DropPosition::Last)) = &self.drop {
                if drop_parent == &current_dir.id {
                    let mut rect = current_dir.row_rect;
                    *rect.bottom_mut() =
                        self.ui.cursor().top() - self.ui.spacing().item_spacing.y * 0.5;
                    self.ui.painter().set(
                        current_dir.drop_marker_idx,
                        RectShape::new(
                            rect,
                            self.ui.visuals().widgets.active.rounding,
                            self.ui.visuals().selection.bg_fill.linear_multiply(0.6),
                            Stroke::NONE,
                        ),
                    );
                }
            }
        }

        if let Some(current_dir) = self.current_dir() {
            if current_dir.is_open {
                let mut p1 = current_dir.icon_rect.center_bottom();
                p1.y += self.ui.spacing().item_spacing.y;
                let mut p2 = p1.clone();
                p2.y = self.ui.cursor().min.y - self.ui.spacing().item_spacing.y;
                self.ui
                    .painter()
                    .line_segment([p1, p2], self.ui.visuals().widgets.noninteractive.bg_stroke);
            }
        }
        self.stack.pop();
    }

    fn row(&mut self, node_config: &mut NodeConfig) -> RowResponse {
        // Load row data
        let row_id = self.ui.id().with(node_config.id).with("row");
        let row_rect = self
            .ui
            .data_mut(|d| d.get_persisted::<Rect>(row_id))
            .unwrap_or(Rect::NOTHING);

        // Interact with the row
        let interaction = self.interact(row_rect, row_id, Sense::click_and_drag());

        if interaction.clicked() {
            *self.selected = Some(node_config.id);
        }

        let background_idx = self.ui.painter().add(Shape::Noop);

        self.drag(&interaction, node_config);
        self.drop(&interaction, node_config);

        let (row_response, icon_response) =
            TreeViewBuilder2::draw_row(self.ui, node_config, self.stack.len() as f32);

        if self.is_selected(&node_config.id) {
            self.ui.painter().set(
                background_idx,
                epaint::RectShape::new(
                    row_response.rect,
                    self.ui.visuals().widgets.active.rounding,
                    self.ui.visuals().selection.bg_fill,
                    Stroke::NONE,
                ),
            );
        }

        // Store row data
        self.ui
            .data_mut(|d| d.insert_persisted(row_id, row_response.rect));

        RowResponse {
            interaction,
            visual: row_response,
            icon: icon_response,
        }
    }

    /// Draw the content as a drag overlay if it is beeing dragged.
    fn drag(&mut self, interaction: &Response, node_config: &mut NodeConfig) {
        if !interaction.dragged_by(PointerButton::Primary)
            && !interaction.drag_released_by(PointerButton::Primary)
        {
            return;
        }

        *self.drag = Some(node_config.id);
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
                let background_position = ui.painter().add(Shape::Noop);

                let (row, _) = TreeViewBuilder2::draw_row(ui, node_config, self.stack.len() as f32);

                ui.painter().set(
                    background_position,
                    epaint::RectShape::new(
                        row.rect,
                        ui.visuals().widgets.active.rounding,
                        ui.visuals().selection.bg_fill.linear_multiply(0.4),
                        Stroke::NONE,
                    ),
                );
                row.rect
            })
            .inner;

        // Move layer to the drag position
        if let Some(pointer_pos) = self.ui.ctx().pointer_interact_pos() {
            let delta = pointer_pos - background_rect.min + drag_offset;
            self.ui.ctx().translate_layer(layer_id, delta);
        }
    }

    fn drop(&mut self, interaction: &Response, node_config: &NodeConfig) {
        if self.current_dir_drop_forbidden() {
            return;
        }
        if self.is_dragged(&node_config.id) {
            return;
        }
        if !self.was_dragged_last_frame {
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

        if let Some(pos) = self.get_drop_position(node_config, drop_quater) {
            self.draw_drop_marker(&pos.1, interaction, node_config.drop_marker_idx);
            *self.drop = Some(pos);
        }
    }

    fn draw_row(
        ui: &mut Ui,
        node_config: &mut NodeConfig,
        depth: f32,
    ) -> (Response, Option<Response>) {
        let InnerResponse {
            inner: icon_response,
            response: row_response,
        } = ui.horizontal(|ui| {
            ui.add_space(ui.spacing().indent * depth);

            let icon_pos = ui.cursor().min;
            if node_config.add_icon.is_some() {
                ui.add_space(ui.spacing().icon_width);
            };

            (node_config.add_content)(ui);
            ui.add_space(ui.available_width());

            node_config.add_icon.as_mut().map(|add_icon| {
                let (small_rect, _) = ui.spacing().icon_rectangles(Rect::from_min_size(
                    icon_pos,
                    vec2(ui.spacing().icon_width, ui.min_size().y),
                ));
                add_icon(ui, small_rect)
            })
        });

        let background_rect = row_response
            .rect
            .expand2(vec2(0.0, ui.spacing().item_spacing.y * 0.5));

        (row_response.with_new_rect(background_rect), icon_response)
    }

    fn draw_drop_marker(
        &mut self,
        drop_position: &DropPosition,
        interaction: &Response,
        marker_idx: ShapeIdx,
    ) {
        pub const DROP_LINE_HEIGHT: f32 = 3.0;

        let drop_marker = match drop_position {
            DropPosition::Before(_) => {
                Rangef::point(interaction.rect.min.y).expand(DROP_LINE_HEIGHT * 0.5)
            }
            DropPosition::First | DropPosition::After(_) => {
                Rangef::point(interaction.rect.max.y).expand(DROP_LINE_HEIGHT * 0.5)
            }
            DropPosition::Last => interaction.rect.y_range(),
        };

        self.ui.painter().set(
            marker_idx,
            epaint::RectShape::new(
                Rect::from_x_y_ranges(interaction.rect.x_range(), drop_marker),
                self.ui.visuals().widgets.active.rounding,
                self.ui
                    .style()
                    .visuals
                    .selection
                    .bg_fill
                    .linear_multiply(0.6),
                Stroke::NONE,
            ),
        );
    }

    /// Interact with the ui without egui adding any extra space.
    fn interact(&mut self, rect: Rect, id: Id, sense: Sense) -> Response {
        let spacing_before = self.ui.spacing().clone();
        self.ui.spacing_mut().item_spacing = Vec2::ZERO;
        let res = self.ui.interact(rect, id, sense);
        *self.ui.spacing_mut() = spacing_before;
        res
    }

    fn current_dir(&self) -> Option<&DirectoryState> {
        if self.stack.is_empty() {
            None
        } else {
            self.stack.last()
        }
    }
    fn current_dir_is_open(&self) -> bool {
        self.current_dir().map_or(true, |dir| dir.is_open)
    }

    fn current_dir_drop_forbidden(&self) -> bool {
        self.current_dir().is_some_and(|dir| dir.drop_forbidden)
    }

    fn is_selected(&self, id: &Uuid) -> bool {
        self.selected
            .as_ref()
            .is_some_and(|selected_id| selected_id == id)
    }

    fn is_dragged(&self, id: &Uuid) -> bool {
        self.drag.as_ref().is_some_and(|drag_id| drag_id == id)
    }

    fn get_drop_position(
        &self,
        node_config: &NodeConfig,
        drop_quater: DropQuater,
    ) -> Option<(Uuid, DropPosition)> {
        let NodeConfig {
            id,
            drop_on_allowed,
            is_open,
            ..
        } = node_config;

        match drop_quater {
            DropQuater::Top => {
                if let Some(parent_dir) = self.current_dir() {
                    return Some((parent_dir.id, DropPosition::Before(*id)));
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
                if let Some(parent_dir) = self.current_dir() {
                    return Some((parent_dir.id, DropPosition::Before(*id)));
                }
                return None;
            }
            DropQuater::MiddleBottom => {
                if *drop_on_allowed {
                    return Some((*id, DropPosition::Last));
                }
                if let Some(parent_dir) = self.current_dir() {
                    return Some((parent_dir.id, DropPosition::After(*id)));
                }
                return None;
            }
            DropQuater::Bottom => {
                if *drop_on_allowed && *is_open {
                    return Some((*id, DropPosition::First));
                }
                if let Some(parent_dir) = self.current_dir() {
                    return Some((parent_dir.id, DropPosition::After(*id)));
                }
                if *drop_on_allowed {
                    return Some((*id, DropPosition::Last));
                }
                return None;
            }
        }
    }
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
}

struct RowResponse {
    interaction: Response,
    visual: Response,
    icon: Option<Response>,
}
