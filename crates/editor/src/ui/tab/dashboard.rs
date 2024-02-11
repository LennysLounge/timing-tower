use bevy_egui::egui::{self, vec2, Align, Direction, Layout, Rect, Sense, Ui};
use unified_sim_model::{model::Entry, Adapter, AdapterCommand};

pub fn dashboard(ui: &mut Ui, adapter: &Adapter) {
    ui.allocate_ui_with_layout(
        ui.available_size_before_wrap(),
        Layout::left_to_right(Align::Min),
        |ui| {
            ui.allocate_ui_at_rect(
                Rect::from_min_size(ui.cursor().min, vec2(300.0, ui.available_height())),
                |ui| {
                    show_entry_table(ui, adapter);
                },
            );
            ui.allocate_ui_with_layout(
                ui.available_size_before_wrap(),
                Layout::top_down(Align::Min),
                |ui| {
                    ui.label("after");
                    ui.label("after");
                },
            )
        },
    );
}

fn show_entry_table(ui: &mut Ui, adapter: &Adapter) {
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
    let height = ui.available_height();
    ui.horizontal(|ui| {
        ui.allocate_space(vec2(0.0, height));
        egui_ltable::Table::new()
            .scroll(false, true)
            .resize_full_height(false)
            .striped(true)
            .column(
                egui_ltable::Column::exact(30.0)
                    .layout(Layout::centered_and_justified(Direction::LeftToRight)),
            )
            .column(egui_ltable::Column::auto().resizeable(true))
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
                                    egui::Label::new(format!("{}", index + 1))
                                        .selectable(false)
                                        .wrap(false),
                                );
                            });
                            row.cell(|ui| {
                                if let Some(driver) = entry.drivers.get(&entry.current_driver) {
                                    ui.add(
                                        egui::Label::new(format!(
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
                                    egui::Label::new(format!("{}", entry.car_number))
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
    });
}
