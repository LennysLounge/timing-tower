use backend::style::{
    assets::AssetDefinition,
    timing_tower::{TimingTower, TimingTowerColumn, TimingTowerRow, TimingTowerTable},
    variables::VariableDefinition,
    StyleDefinition,
};
use egui_ltreeview::TreeViewBuilder;

use super::visitor::{AnyFolder, StyleVisitor};

pub struct TreeViewVisitor<'a> {
    pub builder: TreeViewBuilder<'a>,
}
impl StyleVisitor for TreeViewVisitor<'_> {
    fn visit_style(&mut self, style: &mut StyleDefinition) -> bool {
        self.builder.dir(&style.id, |ui| {
            ui.label("Style");
        });
        true
    }
    fn leave_style(&mut self, _style: &mut StyleDefinition) -> bool {
        self.builder.close_dir();
        true
    }
    fn visit_folder(&mut self, folder: AnyFolder) -> bool {
        self.builder.dir(&folder.id(), |ui| {
            ui.label(folder.name());
        });
        true
    }
    fn leave_folder(&mut self, _folder: AnyFolder) -> bool {
        self.builder.close_dir();
        true
    }
    fn visit_variable(&mut self, variable: &mut VariableDefinition) -> bool {
        self.builder.leaf(&variable.id, |ui| {
            ui.label(&variable.name);
        });
        true
    }
    fn visit_asset(&mut self, asset: &mut AssetDefinition) -> bool {
        self.builder.leaf(&asset.id, |ui| {
            ui.label(&asset.name);
        });
        true
    }
    fn visit_timing_tower(&mut self, timing_tower: &mut TimingTower) -> bool {
        self.builder.dir(&timing_tower.id, |ui| {
            ui.label("Timing tower");
        });
        true
    }
    fn leave_timing_tower(&mut self, _timing_tower: &mut TimingTower) -> bool {
        self.builder.close_dir();
        true
    }
    fn visit_timing_tower_table(&mut self, table: &mut TimingTowerTable) -> bool {
        self.builder.dir(&table.id, |ui| {
            ui.label("Table");
        });
        true
    }
    fn leave_timing_tower_table(&mut self, _table: &mut TimingTowerTable) -> bool {
        self.builder.close_dir();
        true
    }
    fn visit_timing_tower_row(&mut self, row: &mut TimingTowerRow) -> bool {
        self.builder.dir(&row.id, |ui| {
            ui.label("Row");
        });
        true
    }
    fn leave_timing_tower_row(&mut self, _row: &mut TimingTowerRow) -> bool {
        self.builder.close_dir();
        true
    }
    fn visit_timing_tower_column(&mut self, column: &mut TimingTowerColumn) -> bool {
        self.builder.leaf(&column.id, |ui| {
            ui.label(&column.name);
        });
        true
    }
}
