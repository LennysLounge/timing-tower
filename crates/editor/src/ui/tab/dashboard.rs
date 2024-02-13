use backend::{graphic::GraphicStates, style::StyleDefinition};
use bevy_egui::egui::{self, pos2, vec2, Align, Layout, Rect, Sense, Ui};
use unified_sim_model::Adapter;

pub fn dashboard(
    ui: &mut Ui,
    adapter: &Adapter,
    style: &StyleDefinition,
    graphic_states: &mut GraphicStates,
) {
    vertical_movable_split(
        ui,
        300.0,
        |ui| {
            backend::ui::dashboard::show_entry_table(ui, adapter);
        },
        |ui| {
            for graphic in style.graphics.contained_graphics() {
                backend::ui::dashboard::show_graphic(ui, graphic, graphic_states);
            }
        },
    );
}

fn vertical_movable_split(
    ui: &mut Ui,
    initial_width: f32,
    mut left: impl FnMut(&mut Ui),
    right: impl FnMut(&mut Ui),
) {
    let outer_layout = *ui.layout();
    ui.with_layout(
        Layout::left_to_right(Align::Min).with_cross_justify(true),
        |ui| {
            let id = ui.make_persistent_id("__resizeable_rect");
            let width = ui
                .data_mut(|d| d.get_persisted::<f32>(id))
                .unwrap_or(initial_width);
            let rect = Rect::from_min_size(ui.cursor().min, vec2(width, ui.available_height()));

            let interact_rect = Rect::from_min_size(rect.right_top(), vec2(0.0, rect.height()))
                .expand2(vec2(1.0, 0.0));

            let mut child_ui = ui.child_ui(rect, outer_layout);
            left(&mut child_ui);

            let res = ui.interact(interact_rect, id, Sense::drag());
            if res.dragged() || res.hovered() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeHorizontal)
            }

            let new_width = if res.dragged() {
                width + res.drag_delta().x
            } else {
                width
            }
            .max(child_ui.min_rect().width());
            ui.painter().line_segment(
                [
                    pos2(rect.min.x + new_width, rect.min.y),
                    pos2(rect.min.x + new_width, rect.max.y),
                ],
                if res.dragged() || res.hovered() {
                    ui.visuals().widgets.hovered.fg_stroke
                } else {
                    ui.visuals().widgets.noninteractive.bg_stroke
                },
            );
            ui.data_mut(|d| d.insert_persisted(id, new_width));
            ui.allocate_rect(
                Rect::from_min_size(rect.min, vec2(new_width, rect.height())),
                Sense::hover(),
            );
            ui.with_layout(outer_layout, right);
        },
    );
}
