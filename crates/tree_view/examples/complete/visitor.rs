use std::ops::ControlFlow;

use tree_view::v2::TreeViewBuilder;

use crate::data::{Directory, File, NodeVisitor};

pub struct TreeViewVisitor<'a> {
    pub builder: TreeViewBuilder<'a>,
}
impl NodeVisitor for TreeViewVisitor<'_> {
    fn visit_dir(&mut self, dir: &Directory) -> ControlFlow<()> {
        let res = self.builder.dir(&dir.id, |ui| {
            ui.label(&dir.name);
        });
        if let Some(res) = res {
            res.context_menu(|ui| {
                ui.label("Contex menu of a dir");
            });
        }
        ControlFlow::Continue(())
    }

    fn leave_dir(&mut self, _dir: &Directory) -> ControlFlow<()> {
        self.builder.close_dir();
        ControlFlow::Continue(())
    }

    fn visit_file(&mut self, file: &File) -> ControlFlow<()> {
        let res = self.builder.leaf(&file.id, |ui| {
            ui.label(&file.name);
        });
        if let Some(res) = res {
            res.context_menu(|ui| {
                ui.label("Contex menu of a leaf");
            });
        }
        ControlFlow::Continue(())
    }
}
