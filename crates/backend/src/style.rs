use std::ops::ControlFlow;

use enumcapsulate::macros::Encapsulate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    exact_variant::ExactVariant,
    tree_iterator::{Method, TreeItem, TreeIterator, TreeIteratorMut},
};

use self::{
    assets::{AssetDefinition, AssetFolder, AssetOrFolder},
    graphic::{GraphicDefinition, GraphicFolder, GraphicOrFolder},
    scene::SceneDefinition,
    variables::{VariableDefinition, VariableFolder, VariableOrFolder},
};

pub mod assets;
pub mod graphic;
pub mod scene;
pub mod variables;

/// Id that identifies a style item.
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct StyleId(pub Uuid);
impl StyleId {
    pub fn new() -> Self {
        StyleId(Uuid::new_v4())
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct StyleDefinition {
    pub id: StyleId,
    pub assets: Box<ExactVariant<StyleItem, AssetFolder>>,
    pub vars: Box<ExactVariant<StyleItem, VariableFolder>>,
    pub scene: Box<ExactVariant<StyleItem, SceneDefinition>>,
    pub graphics: Box<ExactVariant<StyleItem, GraphicFolder>>,
}
impl Default for StyleDefinition {
    fn default() -> Self {
        Self {
            id: Default::default(),
            assets: Box::new(AssetFolder::new().into()),
            vars: Box::new(VariableFolder::new().into()),
            scene: Box::new(SceneDefinition::new().into()),
            graphics: Box::new(GraphicFolder::new().into()),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Encapsulate)]
#[serde(tag = "style_item_type")]
pub enum StyleItem {
    Style(StyleDefinition),
    Variable(VariableDefinition),
    VariableFolder(VariableFolder),
    Asset(AssetDefinition),
    AssetFolder(AssetFolder),
    Scene(SceneDefinition),
    Graphic(GraphicDefinition),
    GraphicFolder(GraphicFolder),
}

impl TreeItem for StyleItem {
    type Id = StyleId;

    fn id(&self) -> Self::Id {
        match self {
            StyleItem::Style(o) => o.id,
            StyleItem::Variable(o) => o.id,
            StyleItem::VariableFolder(o) => o.id,
            StyleItem::Asset(o) => o.id,
            StyleItem::AssetFolder(o) => o.id,
            StyleItem::Scene(o) => o.id,
            StyleItem::Graphic(o) => o.id,
            StyleItem::GraphicFolder(o) => o.id,
        }
    }
}

impl TreeIterator for StyleItem {
    type Item = StyleItem;

    fn walk<F, R>(&self, f: &mut F) -> ControlFlow<R>
    where
        F: FnMut(&Self::Item, Method) -> ControlFlow<R>,
    {
        f(self, Method::Visit)?;
        match self {
            StyleItem::Style(style) => {
                style.assets.walk(f)?;
                style.vars.walk(f)?;
                style.scene.walk(f)?;
                style.graphics.walk(f)?;
            }
            StyleItem::Variable(_) => (),
            StyleItem::VariableFolder(var_folder) => {
                var_folder.content.iter().try_for_each(|v| match v {
                    VariableOrFolder::Variable(o) => o.walk(f),
                    VariableOrFolder::Folder(o) => o.walk(f),
                })?;
            }
            StyleItem::Asset(_) => (),
            StyleItem::AssetFolder(asset_folder) => {
                asset_folder.content.iter().try_for_each(|v| match v {
                    AssetOrFolder::Asset(o) => o.walk(f),
                    AssetOrFolder::Folder(o) => o.walk(f),
                })?;
            }
            StyleItem::Scene(_) => (),
            StyleItem::Graphic(_) => (),
            StyleItem::GraphicFolder(folder) => {
                folder.content.iter().try_for_each(|v| match v {
                    GraphicOrFolder::Graphic(o) => o.walk(f),
                    GraphicOrFolder::Folder(o) => o.walk(f),
                })?;
            }
        }
        f(self, Method::Leave)
    }
}

impl TreeIteratorMut for StyleItem {
    type Item = StyleItem;

    fn walk_mut<F, R>(&mut self, f: &mut F) -> ControlFlow<R>
    where
        F: FnMut(&mut Self::Item, Method) -> ControlFlow<R>,
    {
        f(self, Method::Visit)?;
        match self {
            StyleItem::Style(style) => {
                style.assets.walk_mut(f)?;
                style.vars.walk_mut(f)?;
                style.scene.walk_mut(f)?;
                style.graphics.walk_mut(f)?;
            }
            StyleItem::Variable(_) => (),
            StyleItem::VariableFolder(var_folder) => {
                var_folder.content.iter_mut().try_for_each(|v| match v {
                    VariableOrFolder::Variable(o) => o.walk_mut(f),
                    VariableOrFolder::Folder(o) => o.walk_mut(f),
                })?;
            }
            StyleItem::Asset(_) => (),
            StyleItem::AssetFolder(asset_folder) => {
                asset_folder.content.iter_mut().try_for_each(|v| match v {
                    AssetOrFolder::Asset(o) => o.walk_mut(f),
                    AssetOrFolder::Folder(o) => o.walk_mut(f),
                })?;
            }
            StyleItem::Scene(_) => (),
            StyleItem::Graphic(_) => (),
            StyleItem::GraphicFolder(folder) => {
                folder.content.iter_mut().try_for_each(|v| match v {
                    GraphicOrFolder::Graphic(o) => o.walk_mut(f),
                    GraphicOrFolder::Folder(o) => o.walk_mut(f),
                })?;
            }
        }
        f(self, Method::Leave)
    }
}
