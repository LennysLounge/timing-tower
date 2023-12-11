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
        // header.response.context_menu(|ui| {
        //     if ui.button("add column").clicked() {
        //         let column = TimingTowerColumn::new();
        //         actions.push(TreeViewAction::Select { node: column.id });
        //         actions.push(TreeViewAction::Insert {
        //             target: self.id,
        //             node: Box::new(column),
        //             position: DropPosition::Last,
        //         });

        //         ui.close_menu();
        //     }
        //     if ui.button("add group").clicked() {
        //         let folder = Folder::<TimingTowerColumn>::new();
        //         actions.push(TreeViewAction::Select { node: folder.id });
        //         actions.push(TreeViewAction::Insert {
        //             target: self.id,
        //             node: Box::new(folder),
        //             position: DropPosition::First,
        //         });
        //         ui.close_menu();
        //     }
        // });
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
        // res.response.context_menu(|ui| {
        //     if ui.button("add column").clicked() {
        //         actions.push(TreeViewAction::Insert {
        //             target: tree_ui.parent_id.unwrap(),
        //             node: Box::new(TimingTowerColumn::new()),
        //             position: DropPosition::After(self.id),
        //         });
        //         ui.close_menu();
        //     }
        //     if ui.button("add group").clicked() {
        //         actions.push(TreeViewAction::Insert {
        //             target: tree_ui.parent_id.unwrap(),
        //             node: Box::new(Folder::<TimingTowerColumn>::new()),
        //             position: DropPosition::After(self.id),
        //         });
        //         ui.close_menu();
        //     }
        //     if ui.button("delete").clicked() {
        //         actions.push(TreeViewAction::Remove { node: self.id });
        //         ui.close_menu();
        //     }
        // });
        ControlFlow::Continue(())
    }

    fn visit_asset(&mut self, asset: &AssetDefinition) -> ControlFlow<()> {
        self.builder.leaf(&asset.id, |ui| {
            ui.label(&asset.name);
        });
        // res.response.context_menu(|ui| {
        //     if ui.button("add image").clicked() {
        //         let image = AssetDefinition {
        //             id: Uuid::new_v4(),
        //             name: String::from("Image"),
        //             value_type: ValueType::Texture,
        //             path: String::new(),
        //         };
        //         actions.push(TreeViewAction::Select { node: *image.id() });
        //         actions.push(TreeViewAction::Insert {
        //             target: tree_ui.parent_id.unwrap(),
        //             node: Box::new(image),
        //             position: tree_view::DropPosition::After(self.id),
        //         });
        //         ui.close_menu();
        //     }
        //     if ui.button("add group").clicked() {
        //         let folder = Folder::<AssetDefinition>::new();
        //         actions.push(TreeViewAction::Select { node: *folder.id() });
        //         actions.push(TreeViewAction::Insert {
        //             target: tree_ui.parent_id.unwrap(),
        //             node: Box::new(folder),
        //             position: tree_view::DropPosition::After(self.id),
        //         });
        //         ui.close_menu();
        //     }
        //     if ui.button("delete").clicked() {
        //         actions.push(TreeViewAction::Remove { node: self.id });
        //         ui.close_menu();
        //     }
        // });
        ControlFlow::Continue(())
    }

    fn visit_variable(&mut self, variable: &VariableDefinition) -> ControlFlow<()> {
        self.builder.leaf(&variable.id, |ui| {
            ui.label(&variable.name);
        });
        // res.response.context_menu(|ui| {
        //     if ui.button("add variable").clicked() {
        //         let var = VariableDefinition::new();
        //         actions.push(TreeViewAction::Select { node: *var.id() });
        //         actions.push(TreeViewAction::Insert {
        //             target: tree_ui.parent_id.unwrap(),
        //             node: Box::new(var),
        //             position: DropPosition::After(*self.id()),
        //         });
        //         ui.close_menu();
        //     }
        //     if ui.button("add group").clicked() {
        //         let folder = Folder::<VariableDefinition>::new();
        //         actions.push(TreeViewAction::Select { node: folder.id });
        //         actions.push(TreeViewAction::Insert {
        //             target: tree_ui.parent_id.unwrap(),
        //             node: Box::new(folder),
        //             position: DropPosition::After(*self.id()),
        //         });
        //         ui.close_menu();
        //     }
        //     if ui.button("delete").clicked() {
        //         actions.push(TreeViewAction::Remove { node: *self.id() });
        //         ui.close_menu();
        //     }
        // });
        ControlFlow::Continue(())
    }
}
