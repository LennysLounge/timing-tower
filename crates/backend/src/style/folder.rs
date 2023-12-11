use std::{any::TypeId, ops::ControlFlow};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::visitor::{NodeVisitor, NodeVisitorMut, StyleNode, Visitable};

pub trait FolderInfo: StyleNode {
    fn name(&self) -> &str;
    fn name_mut(&mut self) -> &mut String;
    fn renameable(&self) -> bool;
    fn as_style_node(&self) -> &dyn StyleNode;
    fn as_style_node_mut(&mut self) -> &mut dyn StyleNode;
    fn content_type_id(&self) -> TypeId;
    fn own_type_id(&self) -> TypeId;
    fn content(&self) -> Vec<&dyn StyleNode>;
    fn remove_index(&mut self, index: usize) -> Option<Box<dyn StyleNode>>;
    fn insert_index(&mut self, index: usize, node: Box<dyn StyleNode>);
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Folder<T> {
    pub id: Uuid,
    pub name: String,
    #[serde(skip)]
    #[serde(default)]
    pub renameable: bool,
    pub content: Vec<FolderOrT<T>>,
}

impl<T> Default for Folder<T> {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: Default::default(),
            content: Default::default(),
            renameable: Default::default(),
        }
    }
}

impl<T> Folder<T>
where
    T: StyleNode,
{
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "new group".to_string(),
            content: Vec::new(),
            renameable: true,
        }
    }
    /// Get a reference to all contained things of this folder recursively.
    pub fn all_t(&self) -> Vec<&T> {
        self.content.iter().flat_map(|c| c.all_t()).collect()
    }

    /// Get a reference to all contained things of this folder recursively.
    pub fn all_t_mut(&mut self) -> Vec<&mut T> {
        self.content
            .iter_mut()
            .flat_map(|c| c.all_t_mut())
            .collect()
    }
}
impl<T> FolderInfo for Folder<T>
where
    T: StyleNode + 'static,
{
    fn name(&self) -> &str {
        &self.name
    }

    fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }

    fn renameable(&self) -> bool {
        self.renameable
    }

    fn as_style_node(&self) -> &dyn StyleNode {
        self
    }

    fn as_style_node_mut(&mut self) -> &mut dyn StyleNode {
        self
    }

    fn content_type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }

    fn own_type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }

    fn content(&self) -> Vec<&dyn StyleNode> {
        self.content
            .iter()
            .map(|x| match x {
                FolderOrT::T(t) => t as &dyn StyleNode,
                FolderOrT::Folder(f) => f,
            })
            .collect()
    }

    fn remove_index(&mut self, index: usize) -> Option<Box<dyn StyleNode>> {
        Some(match self.content.remove(index) {
            FolderOrT::T(t) => Box::new(t),
            FolderOrT::Folder(f) => Box::new(f),
        })
    }

    fn insert_index(&mut self, index: usize, node: Box<dyn StyleNode>) {
        let any = node.to_any();
        if any.is::<Folder<T>>() {
            let folder = any
                .downcast::<Folder<T>>()
                .expect("Cannot downcast but should");
            self.content.insert(index, FolderOrT::Folder(*folder));
        } else if any.is::<T>() {
            let tee = any.downcast::<T>().expect("Cannot downcast but should");
            self.content.insert(index, FolderOrT::T(*tee));
        }
    }
}
impl<T> StyleNode for Folder<T>
where
    T: StyleNode + 'static,
{
    fn id(&self) -> &Uuid {
        &self.id
    }
}
impl<T> Visitable for Folder<T>
where
    T: StyleNode + 'static,
{
    fn walk(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        self.enter(visitor)?;
        self.content.iter().try_for_each(|f| match f {
            FolderOrT::T(t) => t.walk(visitor),
            FolderOrT::Folder(f) => f.walk(visitor),
        })?;
        self.leave(visitor)
    }

    fn enter(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        visitor.visit_folder(self)
    }

    fn leave(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        visitor.leave_folder(self)
    }

    fn walk_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        self.enter_mut(visitor)?;
        self.content.iter_mut().try_for_each(|f| match f {
            FolderOrT::T(t) => t.walk_mut(visitor),
            FolderOrT::Folder(f) => f.walk_mut(visitor),
        })?;
        self.leave_mut(visitor)
    }

    fn enter_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        visitor.visit_folder(self)
    }

    fn leave_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        visitor.leave_folder(self)
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "folder_type")]
pub enum FolderOrT<T> {
    T(T),
    Folder(Folder<T>),
}
impl<T> FolderOrT<T>
where
    T: StyleNode,
{
    pub fn all_t(&self) -> Vec<&T> {
        match self {
            FolderOrT::T(t) => vec![t],
            FolderOrT::Folder(f) => f.all_t(),
        }
    }
    pub fn all_t_mut(&mut self) -> Vec<&mut T> {
        match self {
            FolderOrT::T(t) => vec![t],
            FolderOrT::Folder(f) => f.all_t_mut(),
        }
    }
    pub fn id(&self) -> &Uuid {
        match self {
            FolderOrT::T(t) => &t.id(),
            FolderOrT::Folder(f) => &f.id,
        }
    }
}
