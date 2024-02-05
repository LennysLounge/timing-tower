use std::ops::ControlFlow;

use dyn_clone::DynClone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::tree_iterator::{Method, TreeItem, TreeIterator, TreeIteratorMut};

use self::{
    assets::{AssetDefinition, AssetFolder, AssetOrFolder},
    graphic::{GraphicDefinition, GraphicFolder, GraphicOrFolder},
    scene::SceneDefinition,
    variables::{VariableDefinition, VariableFolder, VariableOrFolder},
};

pub mod assets;
pub mod graphic;
pub mod graphic_items;
pub mod scene;
pub mod variables;

/// Base trait for all elements in the style definition.
pub trait StyleItem: Sync + Send + DynClone {
    fn id(&self) -> &Uuid;
    fn as_ref<'a>(&'a self) -> StyleItemRef<'a>;
    fn as_mut<'a>(&'a mut self) -> StyleItemMut<'a>;
    fn to_owned(self) -> OwnedStyleItem;
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct StyleDefinition {
    pub id: Uuid,
    pub assets: AssetFolder,
    pub vars: VariableFolder,
    pub scene: SceneDefinition,
    pub graphics: GraphicFolder,
}
impl StyleItem for StyleDefinition {
    fn id(&self) -> &Uuid {
        &self.id
    }
    fn as_ref<'a>(&'a self) -> StyleItemRef<'a> {
        StyleItemRef::Style(self)
    }
    fn as_mut<'a>(&'a mut self) -> StyleItemMut<'a> {
        StyleItemMut::Style(self)
    }
    fn to_owned(self) -> OwnedStyleItem {
        OwnedStyleItem::Style(self)
    }
}

pub enum OwnedStyleItem {
    Style(StyleDefinition),
    Variable(VariableDefinition),
    VariableFolder(VariableFolder),
    Asset(AssetDefinition),
    AssetFolder(AssetFolder),
    Scene(SceneDefinition),
    Graphic(GraphicDefinition),
    GraphicFolder(GraphicFolder),
}

impl TreeItem for OwnedStyleItem {
    fn id(&self) -> Uuid {
        match self {
            OwnedStyleItem::Style(o) => o.id,
            OwnedStyleItem::Variable(o) => o.id,
            OwnedStyleItem::VariableFolder(o) => o.id,
            OwnedStyleItem::Asset(o) => o.id,
            OwnedStyleItem::AssetFolder(o) => o.id,
            OwnedStyleItem::Scene(o) => o.id,
            OwnedStyleItem::Graphic(o) => o.id,
            OwnedStyleItem::GraphicFolder(o) => o.id,
        }
    }
}

#[derive(Clone, Copy)]
pub enum StyleItemRef<'a> {
    Style(&'a StyleDefinition),
    Variable(&'a VariableDefinition),
    VariableFolder(&'a VariableFolder),
    Asset(&'a AssetDefinition),
    AssetFolder(&'a AssetFolder),
    Scene(&'a SceneDefinition),
    Graphic(&'a GraphicDefinition),
    GraphicFolder(&'a GraphicFolder),
}

impl TreeItem for StyleItemRef<'_> {
    fn id(&self) -> Uuid {
        match self {
            StyleItemRef::Style(o) => o.id,
            StyleItemRef::Variable(o) => o.id,
            StyleItemRef::VariableFolder(o) => o.id,
            StyleItemRef::Asset(o) => o.id,
            StyleItemRef::AssetFolder(o) => o.id,
            StyleItemRef::Scene(o) => o.id,
            StyleItemRef::Graphic(o) => o.id,
            StyleItemRef::GraphicFolder(o) => o.id,
        }
    }
}

impl TreeIterator for StyleItemRef<'_> {
    type Item<'item> = StyleItemRef<'item>;

    fn walk<F, R>(&self, f: &mut F) -> ControlFlow<R>
    where
        F: FnMut(&Self::Item<'_>, Method) -> ControlFlow<R>,
    {
        f(self, Method::Visit)?;
        match self {
            StyleItemRef::Style(style) => {
                style.assets.as_ref().walk(f)?;
                style.vars.as_ref().walk(f)?;
                style.scene.as_ref().walk(f)?;
                style.graphics.as_ref().walk(f)?;
            }
            StyleItemRef::Variable(_) => (),
            StyleItemRef::VariableFolder(var_folder) => {
                var_folder.content.iter().try_for_each(|v| match v {
                    VariableOrFolder::Variable(o) => o.as_ref().walk(f),
                    VariableOrFolder::Folder(o) => o.as_ref().walk(f),
                })?;
            }
            StyleItemRef::Asset(_) => (),
            StyleItemRef::AssetFolder(asset_folder) => {
                asset_folder.content.iter().try_for_each(|v| match v {
                    AssetOrFolder::Asset(o) => o.as_ref().walk(f),
                    AssetOrFolder::Folder(o) => o.as_ref().walk(f),
                })?;
            }
            StyleItemRef::Scene(_) => (),
            StyleItemRef::Graphic(_) => (),
            StyleItemRef::GraphicFolder(folder) => {
                folder.content.iter().try_for_each(|v| match v {
                    GraphicOrFolder::Graphic(o) => o.as_ref().walk(f),
                    GraphicOrFolder::Folder(o) => o.as_ref().walk(f),
                })?;
            }
        }
        f(self, Method::Leave)
    }
}

pub enum StyleItemMut<'a> {
    Style(&'a mut StyleDefinition),
    Variable(&'a mut VariableDefinition),
    VariableFolder(&'a mut VariableFolder),
    Asset(&'a mut AssetDefinition),
    AssetFolder(&'a mut AssetFolder),
    Scene(&'a mut SceneDefinition),
    Graphic(&'a mut GraphicDefinition),
    GraphicFolder(&'a mut GraphicFolder),
}

impl TreeItem for StyleItemMut<'_> {
    fn id(&self) -> Uuid {
        match self {
            StyleItemMut::Style(o) => o.id,
            StyleItemMut::Variable(o) => o.id,
            StyleItemMut::VariableFolder(o) => o.id,
            StyleItemMut::Asset(o) => o.id,
            StyleItemMut::AssetFolder(o) => o.id,
            StyleItemMut::Scene(o) => o.id,
            StyleItemMut::Graphic(o) => o.id,
            StyleItemMut::GraphicFolder(o) => o.id,
        }
    }
}

impl TreeIteratorMut for StyleItemMut<'_> {
    type Item<'item> = StyleItemMut<'item>;

    fn walk_mut<F, R>(&mut self, f: &mut F) -> ControlFlow<R>
    where
        F: FnMut(&mut StyleItemMut<'_>, Method) -> ControlFlow<R>,
    {
        f(self, Method::Visit)?;
        match self {
            StyleItemMut::Style(style) => {
                style.assets.as_mut().walk_mut(f)?;
                style.vars.as_mut().walk_mut(f)?;
                style.scene.as_mut().walk_mut(f)?;
                style.graphics.as_mut().walk_mut(f)?;
            }
            StyleItemMut::Variable(_) => (),
            StyleItemMut::VariableFolder(var_folder) => {
                var_folder.content.iter_mut().try_for_each(|v| match v {
                    VariableOrFolder::Variable(o) => o.as_mut().walk_mut(f),
                    VariableOrFolder::Folder(o) => o.as_mut().walk_mut(f),
                })?;
            }
            StyleItemMut::Asset(_) => (),
            StyleItemMut::AssetFolder(asset_folder) => {
                asset_folder.content.iter_mut().try_for_each(|v| match v {
                    AssetOrFolder::Asset(o) => o.as_mut().walk_mut(f),
                    AssetOrFolder::Folder(o) => o.as_mut().walk_mut(f),
                })?;
            }
            StyleItemMut::Scene(_) => (),
            StyleItemMut::Graphic(_) => (),
            StyleItemMut::GraphicFolder(folder) => {
                folder.content.iter_mut().try_for_each(|v| match v {
                    GraphicOrFolder::Graphic(o) => o.as_mut().walk_mut(f),
                    GraphicOrFolder::Folder(o) => o.as_mut().walk_mut(f),
                })?;
            }
        }
        f(self, Method::Leave)
    }
}
