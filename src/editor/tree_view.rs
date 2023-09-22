pub trait StyleElement {
    fn get_children(&self) -> Vec<TreeNodeRef>;
    fn get_children_mut(&mut self) -> Vec<TreeNodeRefMut>;
    fn print(&self, depth: usize, id: i32);
    fn mutate(&mut self, value: i32, id_source: &mut IdSource);
}

pub struct ElementTree {
    nodes: Vec<TreeNode<Box<dyn StyleElement>>>,
    id_source: IdSource,
}

pub struct TreeNode<T> {
    pub id: i32,
    pub node: T,
}

pub struct TreeNodeRef<'a> {
    pub id: i32,
    pub node: &'a dyn StyleElement,
}

pub struct TreeNodeRefMut<'a> {
    pub id: i32,
    pub node: &'a mut dyn StyleElement,
}

pub struct IdSource {
    pub id: i32,
}

impl ElementTree {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            id_source: IdSource { id: 0 },
        }
    }

    pub fn add(&mut self, node: TreeNode<Box<dyn StyleElement>>) {
        self.nodes.push(node);
    }

    pub fn next_id(&mut self) -> i32 {
        self.id_source.next()
    }

    pub fn print(&self) {
        self.nodes
            .iter()
            .map(|n| n.as_ref())
            .for_each(|n| n.print(0));
    }

    pub fn mutate(&mut self, value: i32) {
        self.nodes
            .iter_mut()
            .map(|n| n.as_ref_mut())
            .for_each(|mut n| n.mutate(value, &mut self.id_source));
    }
}

impl<T: StyleElement> TreeNode<T> {
    pub fn as_ref(&self) -> TreeNodeRef {
        TreeNodeRef {
            id: self.id,
            node: &self.node,
        }
    }
    pub fn as_ref_mut(&mut self) -> TreeNodeRefMut {
        TreeNodeRefMut {
            id: self.id,
            node: &mut self.node,
        }
    }
}

impl TreeNode<Box<dyn StyleElement>> {
    pub fn as_ref(&self) -> TreeNodeRef {
        TreeNodeRef {
            id: self.id,
            node: &*self.node,
        }
    }
    pub fn as_ref_mut(&mut self) -> TreeNodeRefMut {
        TreeNodeRefMut {
            id: self.id,
            node: &mut *self.node,
        }
    }
}

impl<'a> TreeNodeRef<'a> {
    fn print(&self, depth: usize) {
        self.node.print(depth, self.id);
        self.node.get_children().iter().for_each(|n| {
            n.print(depth + 4);
        });
    }
}

impl<'a> TreeNodeRefMut<'a> {
    fn mutate(&mut self, value: i32, id_source: &mut IdSource) {
        self.node.mutate(value, id_source);
        self.node.get_children_mut().iter_mut().for_each(|n| {
            n.mutate(value, id_source);
        });
    }
}

impl IdSource {
    pub fn next(&mut self) -> i32 {
        self.id += 1
    }
}
