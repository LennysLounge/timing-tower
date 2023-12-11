use std::ops::ControlFlow;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::visitor::{NodeVisitor, NodeVisitorMut, StyleNode, Visitable};

pub trait FolderInfo: StyleNode {
    fn id(&self) -> &Uuid;
    fn name(&self) -> &str;
    fn as_style_node(&self) -> &dyn StyleNode;
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Folder<T> {
    pub id: Uuid,
    pub name: String,
    pub content: Vec<FolderOrT<T>>,
    pub renameable: bool,
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

impl<T> Folder<T> {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "new group".to_string(),
            content: Vec::new(),
            renameable: true,
        }
    }
    /// Get a reference to all contained things of this folder.
    pub fn all_t(&self) -> Vec<&T> {
        self.content.iter().flat_map(|c| c.all_t()).collect()
    }

    /// Get a reference to all contained things of this folder.
    pub fn all_t_mut(&mut self) -> Vec<&mut T> {
        self.content
            .iter_mut()
            .flat_map(|c| c.all_t_mut())
            .collect()
    }
}
impl<T> FolderInfo for Folder<T>
where
    T: Visitable + 'static,
{
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn as_style_node(&self) -> &dyn StyleNode {
        self
    }
}
impl<T> Visitable for Folder<T>
where
    T: Visitable + 'static,
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
pub enum FolderOrT<T> {
    T(T),
    Folder(Folder<T>),
}
impl<T> FolderOrT<T> {
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
}
