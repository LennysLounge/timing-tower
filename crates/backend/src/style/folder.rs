use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
