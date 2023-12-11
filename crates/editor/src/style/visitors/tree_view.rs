use std::ops::ControlFlow;

use backend::style::{definitions::*, visitor::NodeVisitor};
use egui_ltreeview::TreeViewBuilder;

pub struct TreeViewVisitor<'a> {
    pub builder: TreeViewBuilder<'a>,
}
impl NodeVisitor for TreeViewVisitor<'_> {
    fn visit_style(&mut self, style: &StyleDefinition) -> ControlFlow<()> {
        self.builder.dir(&style.id, |ui| {
            ui.label("Style");
        });
        ControlFlow::Continue(())
    }

    fn leave_style(&mut self, _style: &StyleDefinition) -> ControlFlow<()> {
        self.builder.close_dir();
        ControlFlow::Continue(())
    }

    fn visit_folder(&mut self, folder: &dyn FolderInfo) -> ControlFlow<()> {
        self.builder.dir(&folder.id(), |ui| {
            ui.label(folder.name());
        });
        ControlFlow::Continue(())
    }

    fn leave_folder(&mut self, _folder: &dyn FolderInfo) -> ControlFlow<()> {
        self.builder.close_dir();
        ControlFlow::Continue(())
    }

    fn visit_timing_tower(&mut self, tower: &TimingTower) -> ControlFlow<()> {
        self.builder.dir(&tower.id, |ui| {
            ui.label("Timing tower");
        });
        ControlFlow::Continue(())
    }

    fn leave_timing_tower(&mut self, _tower: &TimingTower) -> ControlFlow<()> {
        self.builder.close_dir();
        ControlFlow::Continue(())
    }

    fn visit_timing_tower_table(&mut self, table: &TimingTowerTable) -> ControlFlow<()> {
        self.builder.dir(&table.id, |ui| {
            ui.label("Table");
        });
        ControlFlow::Continue(())
    }

    fn leave_timing_tower_table(&mut self, _table: &TimingTowerTable) -> ControlFlow<()> {
        self.builder.close_dir();
        ControlFlow::Continue(())
    }

    fn visit_timing_tower_row(&mut self, row: &TimingTowerRow) -> ControlFlow<()> {
        self.builder.dir(&row.id, |ui| {
            ui.label("Row");
        });
        ControlFlow::Continue(())
    }

    fn leave_timing_tower_row(&mut self, _row: &TimingTowerRow) -> ControlFlow<()> {
        self.builder.close_dir();
        ControlFlow::Continue(())
    }

    fn visit_timing_tower_column(&mut self, column: &TimingTowerColumn) -> ControlFlow<()> {
        self.builder.leaf(&column.id, |ui| {
            ui.label(&column.name);
        });
        ControlFlow::Continue(())
    }

    fn visit_asset(&mut self, asset: &AssetDefinition) -> ControlFlow<()> {
        self.builder.leaf(&asset.id, |ui| {
            ui.label(&asset.name);
        });
        ControlFlow::Continue(())
    }

    fn visit_variable(&mut self, variable: &VariableDefinition) -> ControlFlow<()> {
        self.builder.leaf(&variable.id, |ui| {
            ui.label(&variable.name);
        });
        ControlFlow::Continue(())
    }
}
