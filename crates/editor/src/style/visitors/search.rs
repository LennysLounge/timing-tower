use std::ops::ControlFlow;

use backend::style::{
    definitions::*,
    visitor::{NodeVisitor, NodeVisitorMut},
    StyleNode,
};
use uuid::Uuid;

pub struct SearchVisitor<'a, T> {
    id: Uuid,
    action: Box<dyn FnMut(&dyn StyleNode) -> T + 'a>,
    output: Option<T>,
}
impl<'a, T> SearchVisitor<'a, T> {
    pub fn new(id: Uuid, action: impl FnMut(&dyn StyleNode) -> T + 'a) -> Self {
        Self {
            id,
            action: Box::new(action),
            output: None,
        }
    }
    pub fn search_in<V>(mut self, node: &V) -> Option<T>
    where
        V: StyleNode,
    {
        node.walk(&mut self);
        self.output
    }
    fn test(&mut self, node: &dyn StyleNode) -> ControlFlow<()> {
        if &self.id == node.id() {
            self.output = Some((self.action)(node));
            ControlFlow::Break(())
        } else {
            ControlFlow::Continue(())
        }
    }
}
impl<'a, T> NodeVisitor for SearchVisitor<'a, T> {
    fn visit_style(&mut self, style: &StyleDefinition) -> ControlFlow<()> {
        self.test(style)
    }

    fn visit_timing_tower(&mut self, tower: &TimingTower) -> ControlFlow<()> {
        self.test(tower)
    }

    fn visit_timing_tower_row(&mut self, row: &TimingTowerRow) -> ControlFlow<()> {
        self.test(row)
    }

    fn visit_timing_tower_column(&mut self, column: &TimingTowerColumn) -> ControlFlow<()> {
        self.test(column)
    }

    fn visit_timing_tower_column_folder(
        &mut self,
        folder: &TimingTowerColumnFolder,
    ) -> ControlFlow<()> {
        self.test(folder)
    }

    fn visit_asset(&mut self, asset: &AssetDefinition) -> ControlFlow<()> {
        self.test(asset)
    }

    fn visit_asset_folder(&mut self, folder: &AssetFolder) -> ControlFlow<()> {
        self.test(folder)
    }

    fn visit_variable(&mut self, variable: &VariableDefinition) -> ControlFlow<()> {
        self.test(variable)
    }

    fn visit_variable_folder(&mut self, folder: &VariableFolder) -> ControlFlow<()> {
        self.test(folder)
    }
    fn visit_clip_area(&mut self, clip_area: &dyn DynClipArea) -> ControlFlow<()> {
        self.test(clip_area.as_style_node())
    }
}

pub struct SearchVisitorMut<'a, T> {
    id: Uuid,
    action: Option<Box<dyn FnOnce(&mut dyn StyleNode) -> T + 'a>>,
    output: Option<T>,
}
impl<'a, T> SearchVisitorMut<'a, T> {
    pub fn new(id: Uuid, action: impl FnOnce(&mut dyn StyleNode) -> T + 'a) -> Self {
        Self {
            id,
            action: Some(Box::new(action)),
            output: None,
        }
    }
    pub fn search_in<V>(mut self, node: &mut V) -> Option<T>
    where
        V: StyleNode,
    {
        node.walk_mut(&mut self);
        self.output
    }
    fn test(&mut self, node: &mut dyn StyleNode) -> ControlFlow<()> {
        if &self.id == node.id() {
            self.output = self.action.take().map(|action| (action)(node));
            ControlFlow::Break(())
        } else {
            ControlFlow::Continue(())
        }
    }
}
impl<'a, T> NodeVisitorMut for SearchVisitorMut<'a, T> {
    fn visit_style(&mut self, style: &mut StyleDefinition) -> ControlFlow<()> {
        self.test(style)
    }

    fn visit_timing_tower(&mut self, tower: &mut TimingTower) -> ControlFlow<()> {
        self.test(tower)
    }

    fn visit_timing_tower_row(&mut self, row: &mut TimingTowerRow) -> ControlFlow<()> {
        self.test(row)
    }

    fn visit_timing_tower_column(&mut self, column: &mut TimingTowerColumn) -> ControlFlow<()> {
        self.test(column)
    }

    fn visit_timing_tower_column_folder(
        &mut self,
        folder: &mut TimingTowerColumnFolder,
    ) -> ControlFlow<()> {
        self.test(folder)
    }

    fn visit_asset(&mut self, asset: &mut AssetDefinition) -> ControlFlow<()> {
        self.test(asset)
    }

    fn visit_asset_folder(&mut self, folder: &mut AssetFolder) -> ControlFlow<()> {
        self.test(folder)
    }

    fn visit_variable(&mut self, variable: &mut VariableDefinition) -> ControlFlow<()> {
        self.test(variable)
    }

    fn visit_variable_folder(&mut self, folder: &mut VariableFolder) -> ControlFlow<()> {
        self.test(folder)
    }

    fn visit_scene(&mut self, scene: &mut SceneDefinition) -> ControlFlow<()> {
        self.test(scene)
    }

    fn visit_clip_area(&mut self, clip_area: &mut dyn DynClipArea) -> ControlFlow<()> {
        self.test(clip_area.as_style_node_mut())
    }
}
