use std::ops::ControlFlow;

use tree_view::v2::TreeViewBuilder;

use crate::data::{Directory, File, Node};

pub trait Visitable {
    fn accept(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()>;
}

impl Visitable for Node {
    fn accept(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        match self {
            Node::Directory(o) => o.accept(visitor),
            Node::File(o) => o.accept(visitor),
        }
    }
}

impl Visitable for Directory {
    fn accept(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        visitor.visit_dir(self)?;
        self.nodes.iter().try_for_each(|n| n.accept(visitor))?;
        visitor.leave_dir(self)
    }
}

impl Visitable for File {
    fn accept(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        visitor.visit_file(self)
    }
}

pub trait NodeVisitor {
    #[allow(unused_variables)]
    fn visit_dir(&mut self, dir: &Directory) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
    #[allow(unused_variables)]
    fn leave_dir(&mut self, dir: &Directory) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
    #[allow(unused_variables)]
    fn visit_file(&mut self, file: &File) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
}

pub struct TreeViewVisitor<'a> {
    builder: TreeViewBuilder<'a>,
}
impl TreeViewVisitor<'_> {
    pub fn run<'a>(visitable: &impl Visitable, builder: TreeViewBuilder<'a>) {
        visitable.accept(&mut TreeViewVisitor { builder });
    }
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
