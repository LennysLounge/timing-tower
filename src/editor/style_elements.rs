use bevy::prelude::Vec2;
use bevy_egui::egui::{collapsing_header::CollapsingState, ComboBox, DragValue, Ui};
use uuid::Uuid;

use crate::style_def::{
    CellStyleDef, ColumnStyleDef, RowStyleDef, TableStyleDef, TextAlignment, TimingTowerStyleDef,
    ValueSource,
};

pub trait StyleElement {
    fn element_tree(&mut self, ui: &mut Ui, selected_element: &mut Option<Uuid>);
    fn find_mut(&mut self, id: &Uuid) -> Option<&mut dyn StyleElement>;
    fn property_editor(&mut self, ui: &mut Ui);
}

pub struct TimingTowerElement {
    pub id: Uuid,
    pub cell: CellStyleDef,
    pub table: TimingTowerTableElement,
}

pub struct TimingTowerTableElement {
    pub id: Uuid,
    pub cell: CellStyleDef,
    pub row_offset: Vec2,
    pub row: TimingTowerRowElement,
}

pub struct TimingTowerRowElement {
    pub id: Uuid,
    pub cell: CellStyleDef,
    pub columns: Vec<TimingTowerColumnElement>,
}

pub struct TimingTowerColumnElement {
    pub id: Uuid,
    pub cell: CellStyleDef,
    pub name: String,
}

impl StyleElement for TimingTowerElement {
    fn element_tree(&mut self, ui: &mut Ui, selected_element: &mut Option<Uuid>) {
        CollapsingState::load_with_default_open(ui.ctx(), ui.next_auto_id(), true)
            .show_header(ui, |ui| {
                let is_selected = selected_element.is_some_and(|uuid| uuid.eq(&self.id));
                if ui.selectable_label(is_selected, "Timing Tower").clicked() {
                    *selected_element = Some(self.id.clone());
                }
            })
            .body(|ui| {
                self.table.element_tree(ui, selected_element);
            });
    }

    fn find_mut(&mut self, id: &Uuid) -> Option<&mut dyn StyleElement> {
        if self.id.eq(id) {
            return Some(self as &mut dyn StyleElement);
        }
        self.table.find_mut(id)
    }

    fn property_editor(&mut self, ui: &mut Ui) {
        cell_style_editor(ui, &mut self.cell);
    }
}

impl TimingTowerElement {
    pub fn from_style_def(style: &TimingTowerStyleDef) -> TimingTowerElement {
        TimingTowerElement {
            id: Uuid::new_v4(),
            cell: style.cell.clone(),
            table: TimingTowerTableElement::from_style_def(&style.table),
        }
    }
    pub fn to_style_def(&self) -> TimingTowerStyleDef {
        TimingTowerStyleDef {
            cell: self.cell.clone(),
            table: self.table.to_style_def(),
        }
    }
}

impl StyleElement for TimingTowerTableElement {
    fn element_tree(&mut self, ui: &mut Ui, selected_element: &mut Option<Uuid>) {
        CollapsingState::load_with_default_open(ui.ctx(), ui.next_auto_id(), true)
            .show_header(ui, |ui| {
                let is_selected = selected_element.is_some_and(|uuid| uuid.eq(&self.id));
                if ui.selectable_label(is_selected, "Table").clicked() {
                    *selected_element = Some(self.id.clone());
                }
            })
            .body(|ui| {
                self.row.element_tree(ui, selected_element);
            });
    }
    fn find_mut(&mut self, id: &Uuid) -> Option<&mut dyn StyleElement> {
        if self.id.eq(id) {
            return Some(self as &mut dyn StyleElement);
        }
        self.row.find_mut(id)
    }
    fn property_editor(&mut self, ui: &mut Ui) {
        ui.label("Row offset:");
        ui.horizontal(|ui| {
            ui.label("Offset x:");
            ui.add(DragValue::new(&mut self.row_offset.x));
        });
        ui.horizontal(|ui| {
            ui.label("Offset y:");
            ui.add(DragValue::new(&mut self.row_offset.y));
        });
        ui.separator();
        cell_style_editor(ui, &mut self.cell);
    }
}

impl TimingTowerTableElement {
    pub fn from_style_def(style: &TableStyleDef) -> Self {
        TimingTowerTableElement {
            id: Uuid::new_v4(),
            cell: style.cell.clone(),
            row_offset: style.row_offset.clone(),
            row: TimingTowerRowElement::from_style_def(&style.row_style),
        }
    }

    pub fn to_style_def(&self) -> TableStyleDef {
        TableStyleDef {
            cell: self.cell.clone(),
            row_offset: self.row_offset.clone(),
            row_style: self.row.to_style_def(),
        }
    }
}

impl StyleElement for TimingTowerRowElement {
    fn element_tree(&mut self, ui: &mut Ui, selected_element: &mut Option<Uuid>) {
        CollapsingState::load_with_default_open(ui.ctx(), ui.next_auto_id(), true)
            .show_header(ui, |ui| {
                let is_selected = selected_element.is_some_and(|uuid| uuid.eq(&self.id));
                if ui.selectable_label(is_selected, "Row").clicked() {
                    *selected_element = Some(self.id.clone());
                }
            })
            .body(|ui| {
                if ui.button("+ Add cell").clicked() {
                    let column = TimingTowerColumnElement {
                        id: Uuid::new_v4(),
                        cell: CellStyleDef::default(),
                        name: "Column".to_string(),
                    };
                    self.columns.push(column);
                }
                for column in self.columns.iter_mut() {
                    column.element_tree(ui, selected_element);
                }
            });
    }
    fn find_mut(&mut self, id: &Uuid) -> Option<&mut dyn StyleElement> {
        if self.id.eq(id) {
            return Some(self as &mut dyn StyleElement);
        }
        self.columns
            .iter_mut()
            .find_map(|element| element.find_mut(id))
    }
    fn property_editor(&mut self, ui: &mut Ui) {
        cell_style_editor(ui, &mut self.cell);
    }
}

impl TimingTowerRowElement {
    pub fn from_style_def(style: &RowStyleDef) -> Self {
        TimingTowerRowElement {
            id: Uuid::new_v4(),
            cell: style.cell.clone(),
            columns: style
                .columns
                .iter()
                .map(|c| TimingTowerColumnElement::from_style_def(c))
                .collect(),
        }
    }

    pub fn to_style_def(&self) -> RowStyleDef {
        RowStyleDef {
            cell: self.cell.clone(),
            columns: {
                let mut columns = Vec::new();
                for column in self.columns.iter() {
                    columns.push(column.to_style_def());
                }
                columns
            },
        }
    }
}

impl StyleElement for TimingTowerColumnElement {
    fn element_tree(&mut self, ui: &mut Ui, selected_element: &mut Option<Uuid>) {
        let is_selected = selected_element.is_some_and(|uuid| uuid.eq(&self.id));
        if ui
            .selectable_label(is_selected, self.name.clone())
            .clicked()
        {
            *selected_element = Some(self.id.clone());
        }
    }

    fn find_mut(&mut self, id: &Uuid) -> Option<&mut dyn StyleElement> {
        self.id.eq(id).then_some(self as &mut dyn StyleElement)
    }

    fn property_editor(&mut self, ui: &mut Ui) {
        ui.label("Name:");
        ui.text_edit_singleline(&mut self.name);
        ui.separator();
        cell_style_editor(ui, &mut self.cell);
    }
}

impl TimingTowerColumnElement {
    pub fn from_style_def(style: &ColumnStyleDef) -> TimingTowerColumnElement {
        TimingTowerColumnElement {
            id: Uuid::new_v4(),
            cell: style.cell.clone(),
            name: style.name.clone(),
        }
    }

    pub fn to_style_def(&self) -> ColumnStyleDef {
        ColumnStyleDef {
            cell: self.cell.clone(),
            name: self.name.clone(),
        }
    }
}

fn cell_style_editor(ui: &mut Ui, style: &mut CellStyleDef) {
    ui.label("Cell:");
    ui.horizontal(|ui| {
        ui.label("Visible:");
        ui.checkbox(&mut style.visible, "");
    });
    ui.horizontal(|ui| {
        ui.label("value source:");
        ComboBox::from_id_source("cell value source")
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
        ComboBox::from_id_source("Text alginment combobox")
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
