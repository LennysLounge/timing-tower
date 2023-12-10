use std::ops::ControlFlow;

use tree_view::v2::TreeViewBuilder;
use uuid::Uuid;

use crate::data::{Directory, File, Node, NodeVisitor, NodeVisitorMut};

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

pub struct RemoveNodeVisitor {
    pub id: Uuid,
    pub removed_node: Option<Node>,
}
impl NodeVisitorMut for RemoveNodeVisitor {
    fn visit_dir(&mut self, dir: &mut Directory) -> ControlFlow<()> {
        println!("visiting: {}", dir.name);
        if let Some(index) = dir.nodes.iter().position(|n| &self.id == n.id()) {
            self.removed_node = Some(dir.nodes.remove(index));
            ControlFlow::Break(())
        } else {
            ControlFlow::Continue(())
        }
    }
    fn visit_file(&mut self, file: &mut File) -> ControlFlow<()> {
        println!("visiting: {}", file.name);
        ControlFlow::Continue(())
    }
}
