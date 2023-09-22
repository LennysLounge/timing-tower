use std::collections::HashMap;

use bevy::prelude::Vec2;
use bevy_egui::egui::{self, collapsing_header::CollapsingState, DragValue, Ui};
use uuid::Uuid;

use crate::style_def::{CellStyleDef, TextAlignment, TimingTowerStyleDef, ValueSource};

pub struct StyleNode {
    id: Uuid,
    element: StyleElement,
}

pub enum StyleElement {
    TimingTower(TimingTower),
    TimingTowerTable(TimingTowerTable),
    TimingTowerRow(TimingTowerRow),
    TimingTowerColumn(TimingTowerColumn),
}

pub struct TimingTower {
    pub cell: CellStyleDef,
    pub table: Box<StyleNode>,
}
pub struct TimingTowerTable {
    pub cell: CellStyleDef,
    pub row_offset: Vec2,
    pub row: Box<StyleNode>,
}
pub struct TimingTowerRow {
    pub cell: CellStyleDef,
    pub columns: HashMap<Uuid, StyleNode>,
}
pub struct TimingTowerColumn {
    pub cell: CellStyleDef,
    pub name: String,
}

impl StyleNode {
    pub fn element_tree(&self, ui: &mut Ui, selected_element: &mut Option<Uuid>) {
        match &self.element {
            StyleElement::TimingTower(o) => o.element_tree(ui, &self.id, selected_element),
            StyleElement::TimingTowerTable(o) => o.element_tree(ui, &self.id, selected_element),
            StyleElement::TimingTowerRow(o) => o.element_tree(ui, &self.id, selected_element),
            StyleElement::TimingTowerColumn(o) => o.element_tree(ui, &self.id, selected_element),
        }
    }
    pub fn find_mut(&mut self, id: &Uuid) -> Option<&mut StyleNode> {
        if &self.id == id {
            Some(self)
        } else {
            match &mut self.element {
                StyleElement::TimingTower(o) => o.find_mut(id),
                StyleElement::TimingTowerTable(o) => o.find_mut(id),
                StyleElement::TimingTowerRow(o) => o.find_mut(id),
                StyleElement::TimingTowerColumn(_) => None,
            }
        }
    }

    pub fn property_editor(&mut self, ui: &mut Ui) {
        match &mut self.element {
            StyleElement::TimingTower(o) => o.property_editor(ui),
            StyleElement::TimingTowerTable(o) => o.property_editor(ui),
            StyleElement::TimingTowerRow(o) => o.property_editor(ui),
            StyleElement::TimingTowerColumn(o) => o.property_editor(ui),
        }
    }
}

impl TimingTower {
    fn element_tree(&self, ui: &mut Ui, id: &Uuid, selected_element: &mut Option<Uuid>) {
        CollapsingState::load_with_default_open(ui.ctx(), ui.next_auto_id(), true)
            .show_header(ui, |ui| {
                let is_selected = selected_element.is_some_and(|uuid| uuid.eq(id));
                if ui.selectable_label(is_selected, "Timing Tower").clicked() {
                    *selected_element = Some(id.clone());
                }
            })
            .body(|ui| {
                let _ = ui.button("+ Add cell");
                self.table.element_tree(ui, selected_element);
            });
    }
    fn find_mut(&mut self, id: &Uuid) -> Option<&mut StyleNode> {
        self.table.find_mut(id)
    }
    fn property_editor(&mut self, ui: &mut Ui) {
        cell_style_editor(ui, &mut self.cell);
    }
}
impl TimingTowerTable {
    fn element_tree(&self, ui: &mut Ui, id: &Uuid, selected_element: &mut Option<Uuid>) {
        CollapsingState::load_with_default_open(ui.ctx(), ui.next_auto_id(), true)
            .show_header(ui, |ui| {
                let is_selected = selected_element.is_some_and(|uuid| uuid.eq(id));
                if ui.selectable_label(is_selected, "Table").clicked() {
                    *selected_element = Some(id.clone());
                }
            })
            .body(|ui| {
                let _ = ui.button("+ Add cell");
                self.row.element_tree(ui, selected_element);
            });
    }
    fn find_mut(&mut self, id: &Uuid) -> Option<&mut StyleNode> {
        self.row.find_mut(id)
    }
    fn property_editor(&mut self, ui: &mut Ui) {
        ui.label("Row offset:");
        ui.horizontal(|ui| {
            ui.label("Offset x:");
            ui.add(egui::DragValue::new(&mut self.row_offset.x));
        });
        ui.horizontal(|ui| {
            ui.label("Offset y:");
            ui.add(egui::DragValue::new(&mut self.row_offset.y));
        });
        ui.separator();
        cell_style_editor(ui, &mut self.cell);
    }
}
impl TimingTowerRow {
    fn element_tree(&self, ui: &mut Ui, id: &Uuid, selected_element: &mut Option<Uuid>) {
        CollapsingState::load_with_default_open(ui.ctx(), ui.next_auto_id(), true)
            .show_header(ui, |ui| {
                let is_selected = selected_element.is_some_and(|uuid| uuid.eq(id));
                if ui.selectable_label(is_selected, "Row").clicked() {
                    *selected_element = Some(id.clone());
                }
            })
            .body(|ui| {
                let _ = ui.button("+ Add cell");
                for column in self.columns.values() {
                    column.element_tree(ui, selected_element);
                }
            });
    }
    fn find_mut(&mut self, id: &Uuid) -> Option<&mut StyleNode> {
        self.columns
            .iter_mut()
            .find_map(|(c_id, c_style)| if c_id == id { Some(c_style) } else { None })
    }
    fn property_editor(&mut self, ui: &mut Ui) {
        cell_style_editor(ui, &mut self.cell);
    }
}
impl TimingTowerColumn {
    fn element_tree(&self, ui: &mut Ui, id: &Uuid, selected_element: &mut Option<Uuid>) {
        let is_selected = selected_element.is_some_and(|uuid| uuid.eq(id));
        if ui
            .selectable_label(is_selected, self.name.clone())
            .clicked()
        {
            *selected_element = Some(id.clone());
        }
    }
    fn property_editor(&mut self, ui: &mut Ui) {
        ui.label("Name:");
        ui.text_edit_singleline(&mut self.name);
        ui.separator();
        cell_style_editor(ui, &mut self.cell);
    }
}

pub fn from_style_def(style: &TimingTowerStyleDef) -> StyleNode {
    let mut columns = HashMap::new();
    for (column_name, column_style) in style.table.row_style.columns.iter() {
        let column = StyleNode {
            id: Uuid::new_v4(),
            element: StyleElement::TimingTowerColumn(TimingTowerColumn {
                cell: column_style.clone(),
                name: column_name.clone(),
            }),
        };
        columns.insert(column.id.clone(), column);
    }
    let row = StyleNode {
        id: Uuid::new_v4(),
        element: StyleElement::TimingTowerRow(TimingTowerRow {
            cell: style.table.row_style.cell.clone(),
            columns,
        }),
    };
    let table = StyleNode {
        id: Uuid::new_v4(),
        element: StyleElement::TimingTowerTable(TimingTowerTable {
            cell: style.table.cell.clone(),
            row_offset: style.table.row_offset.clone(),
            row: Box::new(row),
        }),
    };
    let tower = StyleNode {
        id: Uuid::new_v4(),
        element: StyleElement::TimingTower(TimingTower {
            cell: style.cell.clone(),
            table: Box::new(table),
        }),
    };
    tower
}

fn cell_style_editor(ui: &mut Ui, style: &mut CellStyleDef) {
    ui.label("Cell:");
    ui.horizontal(|ui| {
        ui.label("Visible:");
        ui.checkbox(&mut style.visible, "");
    });
    ui.horizontal(|ui| {
        ui.label("value source:");
        egui::ComboBox::from_id_source("cell value source")
            .selected_text(match &style.value_source {
                ValueSource::FixedValue(_) => "Fixed value",
                ValueSource::DriverName => "Driver name",
                ValueSource::Position => "Position",
                ValueSource::CarNumber => "Car number",
            })
            .show_ui(ui, |ui| {
                if ui
                    .selectable_label(
                        matches!(style.value_source, ValueSource::FixedValue(_)),
                        "Fixed value",
                    )
                    .clicked()
                {
                    style.value_source = ValueSource::FixedValue("".to_string());
                };
                if ui
                    .selectable_label(
                        matches!(style.value_source, ValueSource::DriverName),
                        "Driver name",
                    )
                    .clicked()
                {
                    style.value_source = ValueSource::DriverName;
                };
                if ui
                    .selectable_label(
                        matches!(style.value_source, ValueSource::Position),
                        "Position",
                    )
                    .clicked()
                {
                    style.value_source = ValueSource::Position;
                };
                if ui
                    .selectable_label(
                        matches!(style.value_source, ValueSource::CarNumber),
                        "Car number",
                    )
                    .clicked()
                {
                    style.value_source = ValueSource::CarNumber;
                };
            });
    });
    if let ValueSource::FixedValue(s) = &mut style.value_source {
        ui.horizontal(|ui| {
            ui.label("Text:");
            ui.text_edit_singleline(s);
        });
    }
    ui.horizontal(|ui| {
        ui.label("Text alginment:");
        egui::ComboBox::from_id_source("Text alginment combobox")
            .selected_text(match style.text_alginment {
                TextAlignment::Left => "Left",
                TextAlignment::Center => "Center",
                TextAlignment::Right => "Right",
            })
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut style.text_alginment, TextAlignment::Left, "Left");
                ui.selectable_value(&mut style.text_alginment, TextAlignment::Center, "Center");
                ui.selectable_value(&mut style.text_alginment, TextAlignment::Right, "Right");
            });
    });
    ui.horizontal(|ui| {
        ui.label("Text pos x:");
        ui.add(DragValue::new(&mut style.text_position.x));
    });
    ui.horizontal(|ui| {
        ui.label("Text pos y:");
        ui.add(DragValue::new(&mut style.text_position.y));
    });
    ui.horizontal(|ui| {
        ui.label("Background color:");
        let mut color = style.color.as_rgba_f32();
        ui.color_edit_button_rgba_unmultiplied(&mut color);
        style.color = color.into();
    });
    ui.horizontal(|ui| {
        ui.label("Pos x:");
        ui.add(DragValue::new(&mut style.pos.x));
    });
    ui.horizontal(|ui| {
        ui.label("Pos y:");
        ui.add(DragValue::new(&mut style.pos.y));
    });
    ui.horizontal(|ui| {
        ui.label("Pos z:");
        ui.add(DragValue::new(&mut style.pos.z));
    });
    ui.horizontal(|ui| {
        ui.label("Width:");
        ui.add(DragValue::new(&mut style.size.x).clamp_range(0.0..=f32::MAX));
    });
    ui.horizontal(|ui| {
        ui.label("Height:");
        ui.add(DragValue::new(&mut style.size.y).clamp_range(0.0..=f32::MAX));
    });
    ui.horizontal(|ui| {
        ui.label("Skew:");
        ui.add(DragValue::new(&mut style.skew));
    });
    ui.label("Rounding:");
    ui.horizontal(|ui| {
        ui.label("top left:");
        ui.add(DragValue::new(&mut style.rounding.top_left).clamp_range(0.0..=f32::MAX));
    });
    ui.horizontal(|ui| {
        ui.label("top right:");
        ui.add(DragValue::new(&mut style.rounding.top_right).clamp_range(0.0..=f32::MAX));
    });
    ui.horizontal(|ui| {
        ui.label("bottom right:");
        ui.add(DragValue::new(&mut style.rounding.bot_right).clamp_range(0.0..=f32::MAX));
    });
    ui.horizontal(|ui| {
        ui.label("bottom left:");
        ui.add(DragValue::new(&mut style.rounding.bot_left).clamp_range(0.0..=f32::MAX));
    });
}
