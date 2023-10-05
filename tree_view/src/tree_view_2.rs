use std::ops::RangeInclusive;

use bevy_egui::egui::{
    collapsing_header::CollapsingState,
    epaint::{self},
    layers::ShapeIdx,
    pos2, vec2, Color32, CursorIcon, Id, InnerResponse, LayerId, NumExt, Order, PointerButton,
    Pos2, Rect, Response, Rounding, Sense, Shape, Stroke, Ui, Vec2,
};
use uuid::Uuid;

use crate::split_collapsing_state::SplitCollapsingState;

pub struct TreeUi<'a> {
    pub ui: &'a mut Ui,
    pub bounds: RangeInclusive<f32>,
    context: &'a mut TreeContext,
    parent_id: Option<Uuid>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DropPosition {
    Last { parent_id: Uuid },
    First { parent_id: Uuid },
    After { parent_id: Uuid, after: Uuid },
    Before { parent_id: Uuid, before: Uuid },
}
impl DropPosition {
    pub fn get_parent_node_id(&self) -> &Uuid {
        match self {
            DropPosition::Last { parent_id } => parent_id,
            DropPosition::First { parent_id } => parent_id,
            DropPosition::After { parent_id, .. } => parent_id,
            DropPosition::Before { parent_id, .. } => parent_id,
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct NodeId {
    pub parent_id: Uuid,
    pub node_id: Uuid,
}

#[derive(Clone)]
pub struct DropAction {
    pub from: NodeId,
    pub to: DropPosition,
}

pub struct TreeViewResponse {
    pub response: Response,
    pub selected: Option<Uuid>,
    pub hovered: Option<DropAction>,
    pub dropped: Option<DropAction>,
}

struct TreeContext {
    line_count: i32,
    something_dragged_last_frame: bool,
    selected: Option<Uuid>,
    dragged: Option<NodeId>,
    hovered: Option<DropPosition>,
    drop_disallowed: bool,
}

pub struct TreeView {}
impl TreeView {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(self, ui: &mut Ui, add_content: impl FnOnce(&mut TreeUi)) -> TreeViewResponse {
        // Load state
        let tree_id = ui.make_persistent_id("TreeView");
        ui.ctx().check_for_id_clash(
            tree_id,
            Rect::from_min_size(ui.cursor().min, Vec2::ZERO),
            "Tree view",
        );
        let last_time = ui
            .data_mut(|d| d.get_persisted::<(Option<Uuid>, bool)>(tree_id))
            .unwrap_or((None, false));

        let mut context = TreeContext {
            line_count: 0,
            selected: last_time.0,
            something_dragged_last_frame: last_time.1,
            dragged: None,
            hovered: None,
            drop_disallowed: false,
        };

        let bounds = ui.available_rect_before_wrap().x_range();
        let res = ui.scope(|ui| {
            ui.spacing_mut().item_spacing.y = 7.0;

            ui.allocate_at_least(
                vec2(0.0, -ui.spacing().item_spacing.y / 2.0),
                Sense::hover(),
            );

            let mut tree_ui = TreeUi {
                bounds,
                ui,
                context: &mut context,
                parent_id: None,
            };
            add_content(&mut tree_ui);
            ui.allocate_at_least(
                vec2(ui.available_width(), -ui.spacing().item_spacing.y / 2.0),
                Sense::hover(),
            );
        });

        ui.painter().rect_stroke(
            res.response.rect,
            Rounding::none(),
            Stroke::new(1.0, Color32::BLACK),
        );

        // Store state
        ui.data_mut(|d| {
            d.insert_persisted::<(Option<Uuid>, bool)>(
                tree_id,
                (context.selected, context.dragged.is_some()),
            )
        });

        let drop_action = if let (Some(from), Some(to), false) =
            (context.dragged, context.hovered, context.drop_disallowed)
        {
            Some(DropAction { from, to })
        } else {
            None
        };

        TreeViewResponse {
            response: res.response,
            selected: context.selected,
            hovered: drop_action.clone(),
            dropped: if ui.ctx().input(|i| i.pointer.any_released()) {
                drop_action
            } else {
                None
            },
        }
    }
}

struct NodeConfig {
    id: Uuid,
    drop_on_enabled: bool,
    is_open: bool,
}

pub struct Directory {
    node_config: NodeConfig,
}
impl Directory {
    pub fn new(id: Uuid) -> Self {
        Self {
            node_config: NodeConfig {
                id,
                drop_on_enabled: true,
                is_open: false,
            },
        }
    }

    pub fn show<T1, T2>(
        &mut self,
        tree_ui: &mut TreeUi,
        mut add_header: impl FnMut(&mut Ui) -> T1,
        mut add_body: impl FnMut(&mut TreeUi) -> T2,
    ) -> (InnerResponse<T1>, Option<InnerResponse<T2>>) {
        let collapsing_id = Id::new(self.node_config.id).with("dir header");
        self.node_config.is_open =
            CollapsingState::load_with_default_open(tree_ui.ui.ctx(), collapsing_id, true)
                .is_open();

        let InnerResponse {
            inner: state,
            response: header,
        } = row(tree_ui, &self.node_config, |ui| {
            SplitCollapsingState::show_header(ui, collapsing_id, |ui| add_header(ui))
        });

        let hovered_before = tree_ui.context.hovered.clone();
        let body = state.show_body(tree_ui.ui, |ui| {
            let mut tree_ui = TreeUi {
                ui,
                bounds: tree_ui.bounds.clone(),
                context: tree_ui.context,
                parent_id: Some(self.node_config.id),
            };
            add_body(&mut tree_ui)
        });

        // It is not allowed to drop a parent node onto one of its child nodes
        let drop_is_child_node = tree_ui.context.hovered != hovered_before;
        let parent_is_dragged = tree_ui
            .context
            .dragged
            .as_ref()
            .is_some_and(|node_id| node_id.node_id == self.node_config.id);
        if drop_is_child_node && parent_is_dragged {
            tree_ui.context.drop_disallowed = true;
            tree_ui.ui.ctx().set_cursor_icon(CursorIcon::NoDrop);
        }

        (
            InnerResponse::new(state.header_response.inner, header),
            body,
        )
    }
}

pub struct Leaf {
    node_config: NodeConfig,
}
impl Leaf {
    pub fn new(id: Uuid) -> Self {
        Self {
            node_config: NodeConfig {
                id,
                drop_on_enabled: false,
                is_open: false,
            },
        }
    }
    pub fn show<T>(
        &self,
        tree_ui: &mut TreeUi,
        mut add_header: impl FnMut(&mut Ui) -> T,
    ) -> InnerResponse<T> {
        row(tree_ui, &self.node_config, |ui| {
            ui.horizontal(|ui| add_header(ui)).inner
        })
    }
}

fn row<T>(
    tree_ui: &mut TreeUi,
    node: &NodeConfig,
    mut add_content: impl FnMut(&mut Ui) -> T,
) -> InnerResponse<T> {
    tree_ui.context.line_count += 1;
    let is_selected = tree_ui
        .context
        .selected
        .is_some_and(|sel_id| sel_id == node.id);
    let is_even = tree_ui.context.line_count % 2 == 0;

    let row_background = tree_ui.ui.painter().add(Shape::Noop);
    let hover_background = tree_ui.ui.painter().add(Shape::Noop);

    let (interaction, row) = row_interaction(tree_ui, node.id, |ui| add_content(ui));
    if interaction.clicked() || interaction.dragged() {
        tree_ui.context.selected = Some(node.id);
    }

    draw_drag_overlay(tree_ui, node.id, &interaction, &row, |ui| {
        add_content(ui);
    });

    drop_targets(tree_ui, node, &row, hover_background);

    tree_ui.ui.painter().set(
        row_background,
        epaint::RectShape {
            rect: row.response.rect,
            rounding: tree_ui.ui.visuals().widgets.active.rounding,
            fill: if is_selected {
                tree_ui.ui.style().visuals.selection.bg_fill
            } else if is_even {
                Color32::from_rgba_premultiplied(10, 10, 10, 0)
            } else {
                Color32::TRANSPARENT
            },
            stroke: Stroke::NONE,
        },
    );

    InnerResponse::new(row.inner.inner, interaction)
}

/// Adds a row with the width of the bounds that can be clicked or dragged.
fn row_interaction<T>(
    tree_ui: &mut TreeUi,
    id: Uuid,
    add_content: impl FnOnce(&mut Ui) -> T,
) -> (Response, InnerResponse<InnerResponse<T>>) {
    // Interact with the background first. If we tryed to interact with the background
    // after the element has been drawn we would take over all of the interaction for
    // the given area and the element would never be allowed to interact.
    // Do this this right we need to remember the size of the background area from
    // last frame.
    let interact_id = Id::new(id).with("row background interaction");
    let interact_rect = tree_ui
        .ui
        .data_mut(|d| d.get_persisted::<Rect>(interact_id))
        .unwrap_or(Rect::NOTHING);
    // The `interact` will add some space to the rect. To get exact interaction we
    // need to take that increase away.
    let interact_rect = interact_rect.expand2(
        (0.5 * tree_ui.ui.spacing().item_spacing - Vec2::splat(0.1))
            .at_least(Vec2::splat(0.0))
            .at_most(Vec2::splat(5.0))
            * -1.0,
    );
    let interact_res = tree_ui
        .ui
        .interact(interact_rect, interact_id, Sense::click_and_drag());

    let res = draw_content_at_full_size(tree_ui, add_content);

    tree_ui
        .ui
        .data_mut(|d| d.insert_persisted(interact_id, res.response.rect));

    (interact_res, res)
}

/// Draw the content as a drag overlay if it is beeing dragged.
fn draw_drag_overlay<T, U>(
    tree_ui: &mut TreeUi,
    id: Uuid,
    interaction: &Response,
    row: &InnerResponse<InnerResponse<T>>,
    add_content: impl FnOnce(&mut Ui) -> U,
) {
    let TreeUi {
        ui,
        bounds,
        context,
        parent_id,
    } = tree_ui;

    let drag_source_id = ui.make_persistent_id("Drag source");

    let drag_offset = if interaction.drag_started_by(PointerButton::Primary) {
        ui.ctx()
            .pointer_latest_pos()
            .map(|pointer_pos| row.response.rect.min - pointer_pos)
            .unwrap_or(Vec2::ZERO)
    } else {
        ui.data_mut(|d| d.get_persisted::<Vec2>(drag_source_id))
            .unwrap_or(Vec2::ZERO)
    };

    if interaction.dragged_by(PointerButton::Primary)
        || interaction.drag_released_by(PointerButton::Primary)
    {
        // A node without a parent is a root node and cannot be dragged.
        if let Some(parent_id) = parent_id {
            context.dragged = Some(NodeId {
                parent_id: *parent_id,
                node_id: id,
            });
            ui.ctx().set_cursor_icon(CursorIcon::Grabbing);
        } else {
            ui.ctx().set_cursor_icon(CursorIcon::NoDrop);
        }

        // Paint the content again to a new layer for the drag overlay.
        let layer_id = LayerId::new(Order::Tooltip, drag_source_id);
        let background_rect = ui
            .child_ui(ui.available_rect_before_wrap(), *ui.layout())
            .with_layer_id(layer_id, |ui| {
                let background = ui.painter().add(Shape::Noop);

                let mut tree_ui = TreeUi {
                    ui,
                    bounds: bounds.clone(),
                    context: context,
                    parent_id: None,
                };
                let res = draw_content_at_full_size(&mut tree_ui, add_content);

                ui.painter().set(
                    background,
                    epaint::RectShape {
                        rect: res.response.rect,
                        rounding: ui.visuals().widgets.active.rounding,
                        fill: ui.visuals().selection.bg_fill.linear_multiply(0.5),
                        stroke: Stroke::NONE,
                    },
                );
                res
            })
            .inner
            .response;

        // Move layer to the drag position
        if let Some(pointer_pos) = ui.ctx().pointer_interact_pos() {
            let delta = pointer_pos - background_rect.rect.min + drag_offset;
            ui.ctx().translate_layer(layer_id, delta);
        }
    }

    ui.data_mut(|d| d.insert_persisted::<Vec2>(drag_source_id, drag_offset));
}

/// Draws the content and extends their rectangles to the full width of the
/// Tree. The first (inner) `InnerResponse` expands the rectangle to the
/// right side of the tree. The second (outer) `InnerResponse` expands
/// the rect to the left side of the tree.
fn draw_content_at_full_size<T>(
    tree_ui: &mut TreeUi,
    add_content: impl FnOnce(&mut Ui) -> T,
) -> InnerResponse<InnerResponse<T>> {
    let TreeUi { ui, bounds, .. } = tree_ui;

    // Show the element.
    let scope = ui.scope(|ui| {
        let res = ui.scope(|ui| add_content(ui));

        let background_to_right = Rect::from_min_max(
            res.response.rect.min,
            pos2(*bounds.end(), res.response.rect.max.y),
        )
        .expand2(vec2(0.0, ui.spacing().item_spacing.y / 2.0));
        InnerResponse::new(res.inner, res.response.with_new_rect(background_to_right))
    });

    let background_full_width =
        Rect::from_x_y_ranges(bounds.clone(), scope.response.rect.y_range())
            .expand2(vec2(0.0, ui.spacing().item_spacing.y / 2.0));

    InnerResponse::new(
        scope.inner,
        scope.response.with_new_rect(background_full_width),
    )
}

pub const DROP_LINE_HEIGHT: f32 = 3.0;
pub const DROP_LINE_HOVER_HEIGHT: f32 = 5.0;

fn drop_targets<T>(
    tree_ui: &mut TreeUi,
    node: &NodeConfig,
    row: &InnerResponse<InnerResponse<T>>,
    background_pos: ShapeIdx,
) {
    let TreeUi {
        ui,
        context,
        parent_id,
        ..
    } = tree_ui;

    // If there is nothing dragged we dont have to worry about dropping anything either.
    if !context.something_dragged_last_frame {
        return;
    }

    // We dont want to allow dropping on the selected item.
    if context
        .selected
        .is_some_and(|selected_id| node.id == selected_id)
    {
        return;
    }

    let rect = row.response.rect;

    let drop_id = ui.make_persistent_id("Drop target");
    let res = ui.interact(rect, drop_id, Sense::hover());

    let Some(Pos2 { y, .. }) = res.hover_pos() else {
        return;
    };

    // The `interact` adds a bit of space around the rect to make interaction easier.
    // This causes the row above and below to also be hovered when they shouldnt be.
    // Check to make sure we are really only hovering on our rect.
    if y < row.response.rect.top() || y >= row.response.rect.bottom() {
        return;
    }

    let h0 = rect.min.y;
    let h1 = rect.min.y + DROP_LINE_HOVER_HEIGHT;
    let h2 = (rect.min.y + rect.max.y) / 2.0;
    let h3 = rect.max.y - DROP_LINE_HOVER_HEIGHT;
    let h4 = rect.max.y;

    let line_above = rect.min.y - DROP_LINE_HEIGHT / 2.0..=rect.min.y + DROP_LINE_HEIGHT / 2.0;
    let line_below = rect.max.y - DROP_LINE_HEIGHT / 2.0..=rect.max.y + DROP_LINE_HEIGHT / 2.0;
    let line_background = h0..=h4;

    let hover_result = match y {
        y if y >= h0 && y < h1 => parent_id.map(|parent_id| {
            (
                line_above,
                DropPosition::Before {
                    parent_id,
                    before: node.id,
                },
            )
        }),
        y if y >= h1 && y < h2 => {
            if node.drop_on_enabled {
                Some((line_background, DropPosition::Last { parent_id: node.id }))
            } else {
                parent_id.map(|parent_id| {
                    (
                        line_above,
                        DropPosition::Before {
                            parent_id,
                            before: node.id,
                        },
                    )
                })
            }
        }
        y if y >= h2 && y < h3 => {
            if node.drop_on_enabled {
                Some((line_background, DropPosition::Last { parent_id: node.id }))
            } else {
                parent_id.map(|parent_id| {
                    (
                        line_below,
                        DropPosition::After {
                            parent_id,
                            after: node.id,
                        },
                    )
                })
            }
        }
        y if y >= h3 && y < h4 => {
            if node.is_open {
                Some((line_below, DropPosition::First { parent_id: node.id }))
            } else {
                parent_id.map(|parent_id| {
                    (
                        line_below,
                        DropPosition::After {
                            parent_id,
                            after: node.id,
                        },
                    )
                })
            }
        }
        _ => unreachable!(),
    };
    if let Some((y_range, drop_action)) = hover_result {
        context.hovered = Some(drop_action);
        tree_ui.ui.painter().set(
            background_pos,
            epaint::RectShape {
                rect: Rect::from_x_y_ranges(row.inner.response.rect.x_range(), y_range),
                rounding: tree_ui.ui.visuals().widgets.active.rounding,
                fill: tree_ui.ui.style().visuals.selection.bg_fill,
                stroke: Stroke::NONE,
            },
        );
    }
}
