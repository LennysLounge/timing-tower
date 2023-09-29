use bevy_egui::egui::{
    collapsing_header::CollapsingState, epaint, pos2, vec2, Color32, Rect, Rounding, Sense, Shape,
    Stroke, Ui,
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
            show_node(
                ui,
                root,
                ui.available_rect_before_wrap(),
                &mut 0,
                &mut self.selected,
            );
        });

        ui.painter().rect_stroke(
            res.response.rect,
            Rounding::none(),
            Stroke::new(1.0, Color32::BLACK),
        );
    }
}

fn show_node(
    ui: &mut Ui,
    node: &dyn TreeNode,
    bounds: Rect,
    line: &mut i32,
    selected: &mut Option<Uuid>,
) {
    let where_to_put_background = ui.painter().add(Shape::Noop);
    *line += 1;
    let highlighted = *line % 2 == 0;
    let is_selected = selected.is_some_and(|id| &id == node.get_id());

    let rect = if node.is_directory() {
        let collapsing_id = ui.next_auto_id();
        let (button, header, _) =
            CollapsingState::load_with_default_open(ui.ctx(), collapsing_id, true)
                .show_header(ui, |ui| {
                    node.show(ui);
                    ui.allocate_at_least(vec2(ui.available_width(), 0.0), Sense::hover());
                })
                .body(|ui| {
                    for child in node.get_children() {
                        show_node(ui, child, bounds, line, selected);
                    }
                });

        let left_of_button = Rect::from_min_max(
            pos2(bounds.left(), header.response.rect.top()),
            pos2(button.rect.left(), header.response.rect.bottom()),
        );

        let right_of_button = Rect::from_min_max(
            pos2(button.rect.right(), header.response.rect.top()),
            pos2(bounds.right(), header.response.rect.bottom()),
        );

        let right_of_button = ui.interact(right_of_button, collapsing_id.with(1), Sense::click());
        let left_of_button = ui.interact(left_of_button, collapsing_id.with(2), Sense::click());
        if right_of_button.double_clicked() || left_of_button.double_clicked() {
            if let Some(mut state) = CollapsingState::load(ui.ctx(), collapsing_id) {
                state.toggle(ui);
                state.store(ui.ctx());
            }
        }

        if right_of_button.clicked() || left_of_button.clicked() {
            *selected = Some(*node.get_id());
        }

        Rect::from_min_max(
            pos2(bounds.left(), header.response.rect.top()),
            pos2(bounds.right(), header.response.rect.bottom()),
        )
    } else {
        let res = ui.scope(|ui| {
            ui.horizontal(|ui| {
                node.show(ui);
                ui.allocate_at_least(vec2(ui.available_width(), 0.0), Sense::hover());
            });
        });
        let full_width = Rect::from_min_max(
            pos2(bounds.left(), res.response.rect.top()),
            pos2(bounds.right(), res.response.rect.bottom()),
        );
        if ui
            .interact(full_width, res.response.id.with(1), Sense::click())
            .clicked()
        {
            *selected = Some(*node.get_id());
        }
        full_width
    };

    ui.painter().set(
        where_to_put_background,
        epaint::RectShape {
            rect,
            rounding: Rounding::none(),
            fill: if is_selected {
                ui.style().visuals.selection.bg_fill
            } else if highlighted {
                Color32::from_rgba_premultiplied(10, 10, 10, 0)
                //ui.style().visuals.faint_bg_color
            } else {
                Color32::TRANSPARENT
            },
            stroke: Stroke::NONE,
        },
    );

    // ui.painter()
    //     .rect_stroke(rect, Rounding::none(), Stroke::new(1.0, Color32::BLACK));
}
