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

        ui.group(|ui| {
            let tree_response =
                TreeView::new(ui.make_persistent_id("__graphic").with(graphic.id.0))
                    .min_width(150.0)
                    .show(ui, |mut builder| {
                        // if let Some(state) = graphic_states.states.get(&graphic.id) {
                        //     builder.set_selected(*state);
                        // }
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

pub fn show_entry_table(ui: &mut Ui, adapter: Option<&Adapter>) {
    let Some(adapter) = adapter else {
        return;
    };

    let model = adapter.model.read_raw();

    // Get entries sorted by position
    let mut entries: Vec<&Entry> = model
        .current_session()
        .map(|session| session.entries.values().collect::<Vec<_>>())
        .unwrap_or(Vec::new());
    entries.sort_by_key(|e| *e.position);

    egui_ltable::Table::new()
        .scroll(false, true)
        .resize_full_height(false)
        .striped(true)
        .column(
            egui_ltable::Column::exact(10.0)
                .layout(Layout::centered_and_justified(Direction::LeftToRight)),
        )
        .column(
            egui_ltable::Column::exact(30.0)
                .layout(Layout::centered_and_justified(Direction::LeftToRight)),
        )
        .column(egui_ltable::Column::exact(ui.available_width() - 210.0))
        .column(
            egui_ltable::Column::exact(30.0)
                .layout(Layout::centered_and_justified(Direction::LeftToRight)),
        )
        .column(
            egui_ltable::Column::exact(100.0)
                .layout(Layout::centered_and_justified(Direction::LeftToRight)),
        )
        .show(ui, |table| {
            table.row(egui_ltable::Row::new().height(30.0).fixed(true), |row| {
                row.cell(|_| {});
                row.cell(|ui| {
                    ui.heading("#");
                });
                row.cell(|ui| {
                    ui.heading("Name");
                });
                row.cell(|ui| {
                    ui.heading("Nr");
                });
                row.cell(|ui| {
                    ui.heading("Interval");
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
                            if *entry.in_pits {
                                ui.label("P");
                            }
                        });
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
                        row.cell(|ui| {
                            if let Some(time) = entry.time_behind_position_ahead.get_available() {
                                ui.add(Label::new(time.format()).selectable(false).wrap(false));
                            } else {
                                ui.add(Label::new("-").selectable(false).wrap(false));
                            }
                        });
                    },
                );
                if r.clicked() {
                    adapter.send(AdapterCommand::FocusOnCar(entry.id));
                }
            }
        });
}
