use super::tree_view::{IdSource, StyleElement, TreeNode, TreeNodeRef, TreeNodeRefMut};

pub struct Number {
    pub value: i32,
    pub special: Option<TreeNode<Special>>,
    pub nodes: Vec<TreeNode<Box<dyn StyleElement>>>,
}

impl StyleElement for Number {
    fn get_children(&self) -> Vec<TreeNodeRef> {
        let mut v = Vec::new();
        if let Some(s) = &self.special {
            v.push(s.as_ref());
        }
        v.extend(self.nodes.iter().map(|n| n.as_ref()));
        v
    }

    fn get_children_mut(&mut self) -> Vec<TreeNodeRefMut> {
        let mut v = Vec::new();
        if let Some(s) = &mut self.special {
            v.push(s.as_ref_mut());
        }
        v.extend(self.nodes.iter_mut().map(|n| n.as_ref_mut()));
        v
    }

    fn print(&self, depth: usize, id: i32) {
        println!("{:>depth$}Number({}) id({id})", "", self.value);
    }

    fn mutate(&mut self, value: i32, id_source: &mut IdSource) {
        if self.value == value {
            self.value += 1;

            self.nodes.push(TreeNode {
                id: id_source.next(),
                node: Box::new(Number {
                    value: 0,
                    special: None,
                    nodes: Vec::new(),
                }),
            });
        }
    }
}

pub struct Special {}
impl StyleElement for Special {
    fn get_children(&self) -> Vec<TreeNodeRef> {
        Vec::new()
    }

    fn get_children_mut(&mut self) -> Vec<TreeNodeRefMut> {
        Vec::new()
    }

    fn print(&self, depth: usize, id: i32) {
        println!("{:>depth$}Special id({id})", "");
    }
    fn mutate(&mut self, value: i32, id_source: &mut IdSource) {}
}
