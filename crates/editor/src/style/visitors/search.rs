use std::ops::ControlFlow;

use backend::style::{
    definitions::*,
    visitor::{Node, NodeVisitor, NodeVisitorMut},
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
    fn visit(&mut self, node: Node) -> ControlFlow<()> {
        match node {
            Node::Style(o) => self.test(o),
            Node::Variable(o) => self.test(o),
            Node::VariableFolder(o) => self.test(o),
            Node::Asset(o) => self.test(o),
            Node::AssetFolder(o) => self.test(o),
            Node::Scene(o) => self.test(o),
            Node::TimingTower(o) => self.test(o),
            Node::TimingTowerRow(o) => self.test(o),
            Node::TimingTowerColumn(o) => self.test(o),
            Node::TimingTowerColumnFolder(o) => self.test(o),
            Node::ClipArea(o) => self.test(o),
        }
    }

    fn leave(&mut self, _node: Node) -> ControlFlow<()> {
        ControlFlow::Continue(())
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
