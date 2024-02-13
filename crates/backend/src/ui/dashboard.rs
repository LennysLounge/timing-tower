use bevy_egui::egui::{Direction, Label, Layout, Sense, Ui};
use egui_ltreeview::{Action, TreeView};
use unified_sim_model::{model::Entry, Adapter, AdapterCommand};

use crate::{
    graphic::GraphicStates,
    style::graphic::{GraphicDefinition, GRAPHIC_STATE_HIDDEN},
};

pub fn show_graphic(ui: &mut Ui, graphic: &GraphicDefinition, graphic_states: &mut GraphicStates) {
    ui.group(|ui| {
        ui.heading(&graphic.name);

        let min_width = ui.min_size().x;
        ui.group(|ui| {
            let tree_response =
                TreeView::new(ui.make_persistent_id("__graphic").with(graphic.id.0))
                    .max_width(if min_width > 150.0 {
                        Some(min_width)
                    } else {
                        Some(150.0)
                    })
                    .fill_space_horizontal(true)
                    .show(ui, |mut builder| {
                        if let Some(state) = graphic_states.states.get(&graphic.id) {
                            builder.set_selected(*state);
                        }
                        for state in graphic.states.iter() {
                            builder.leaf(state.id, &state.name);
                        }
                        builder.leaf(GRAPHIC_STATE_HIDDEN, "Hidden");
                    });
            for action in tree_response.actions {
                if let Action::SetSelected(Some(selected_state)) = action {
                    graphic_states.states.insert(graphic.id, selected_state);
                }
            }
        });
    });
}

pub fn show_entry_table(ui: &mut Ui, adapter: &Adapter) {
    let model = adapter.model.read().expect("Cannot lock model for reading");
    let current_session = model.current_session().unwrap();

    // Get entries sorted by position
    let mut entries: Vec<&Entry> = current_session.entries.values().collect();
    entries.sort_by(|e1, e2| {
        let is_connected = e2.connected.cmp(&e1.connected);
        let position = e1
            .position
            .partial_cmp(&e2.position)
            .unwrap_or(std::cmp::Ordering::Equal);
        is_connected.then(position)
    });
    egui_ltable::Table::new()
        .scroll(false, true)
        .resize_full_height(false)
        .striped(true)
        .column(
            egui_ltable::Column::exact(30.0)
                .layout(Layout::centered_and_justified(Direction::LeftToRight)),
        )
        .column(egui_ltable::Column::exact(ui.available_width() - 80.0))
        .column(
            egui_ltable::Column::exact(30.0)
                .layout(Layout::centered_and_justified(Direction::LeftToRight)),
        )
        .show(ui, |table| {
            table.row(egui_ltable::Row::new().height(30.0).fixed(true), |row| {
                row.cell(|ui| {
                    ui.heading("#");
                });
                row.cell(|ui| {
                    ui.heading("Name");
                });
                row.cell(|ui| {
                    ui.heading("Nr");
                });
            });
            for (index, entry) in entries.iter().enumerate() {
                let r = table.row(
                    egui_ltable::Row::new()
                        .height(22.0)
                        .sense(Sense::click())
                        .hover_highlight(true)
                        .highlight(entry.focused),
                    |row| {
                        row.cell(|ui| {
                            ui.add(
                                Label::new(format!("{}", index + 1))
                                    .selectable(false)
                                    .wrap(false),
                            );
                        });
                        row.cell(|ui| {
                            if let Some(driver) = entry.drivers.get(&entry.current_driver) {
                                ui.add(
                                    Label::new(format!(
                                        "{} {}",
                                        driver.first_name, driver.last_name
                                    ))
                                    .selectable(false)
                                    .truncate(true),
                                );
                            }
                        });
                        row.cell(|ui| {
                            ui.add(
                                Label::new(format!("{}", entry.car_number))
                                    .selectable(false)
                                    .wrap(false),
                            );
                        });
                    },
                );
                if r.clicked() {
                    adapter.send(AdapterCommand::FocusOnCar(entry.id));
                }
            }
        });
}
