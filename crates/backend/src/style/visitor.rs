use std::ops::ControlFlow;

use super::{
    assets::{AssetDefinition, AssetFolder},
    clip_area::DynClipArea,
    scene::SceneDefinition,
    timing_tower::{TimingTower, TimingTowerColumn, TimingTowerColumnFolder, TimingTowerRow},
    variables::{VariableDefinition, VariableFolder},
    StyleDefinition,
};

pub trait Visitable {
    fn walk(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()>;
    fn enter(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()>;
    fn leave(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()>;
    fn walk_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()>;
    fn enter_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()>;
    fn leave_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()>;
}

pub trait NodeVisitor {
    #[allow(unused_variables)]
    fn visit_style(&mut self, style: &StyleDefinition) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
    #[allow(unused_variables)]
    fn leave_style(&mut self, style: &StyleDefinition) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
    #[allow(unused_variables)]
    fn visit_timing_tower(&mut self, tower: &TimingTower) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
    #[allow(unused_variables)]
    fn leave_timing_tower(&mut self, tower: &TimingTower) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
    #[allow(unused_variables)]
    fn visit_timing_tower_row(&mut self, row: &TimingTowerRow) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
    #[allow(unused_variables)]
    fn leave_timing_tower_row(&mut self, row: &TimingTowerRow) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
    #[allow(unused_variables)]
    fn visit_timing_tower_column(&mut self, column: &TimingTowerColumn) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
    #[allow(unused_variables)]
    fn visit_timing_tower_column_folder(
        &mut self,
        folder: &TimingTowerColumnFolder,
    ) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
    #[allow(unused_variables)]
    fn leave_timing_tower_column_folder(
        &mut self,
        folder: &TimingTowerColumnFolder,
    ) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
    #[allow(unused_variables)]
    fn visit_asset(&mut self, asset: &AssetDefinition) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
    #[allow(unused_variables)]
    fn visit_asset_folder(&mut self, folder: &AssetFolder) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
    #[allow(unused_variables)]
    fn leave_asset_folder(&mut self, folder: &AssetFolder) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
    #[allow(unused_variables)]
    fn visit_variable(&mut self, variable: &VariableDefinition) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
    #[allow(unused_variables)]
    fn visit_variable_folder(&mut self, folder: &VariableFolder) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
    #[allow(unused_variables)]
    fn leave_variable_folder(&mut self, folder: &VariableFolder) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
    #[allow(unused_variables)]
    fn visit_scene(&mut self, scene: &SceneDefinition) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
    #[allow(unused_variables)]
    fn leave_scene(&mut self, scene: &SceneDefinition) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
    #[allow(unused_variables)]
    fn visit_clip_area(&mut self, clip_area: &dyn DynClipArea) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
    #[allow(unused_variables)]
    fn leave_clip_area(&mut self, clip_area: &dyn DynClipArea) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
}

pub trait NodeVisitorMut {
    #[allow(unused_variables)]
    fn visit_style(&mut self, style: &mut StyleDefinition) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
    #[allow(unused_variables)]
    fn leave_style(&mut self, style: &mut StyleDefinition) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
    #[allow(unused_variables)]
    fn visit_timing_tower(&mut self, tower: &mut TimingTower) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
    #[allow(unused_variables)]
    fn leave_timing_tower(&mut self, tower: &mut TimingTower) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
    #[allow(unused_variables)]
    fn visit_timing_tower_row(&mut self, row: &mut TimingTowerRow) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
    #[allow(unused_variables)]
    fn leave_timing_tower_row(&mut self, row: &mut TimingTowerRow) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
    #[allow(unused_variables)]
    fn visit_timing_tower_column(&mut self, column: &mut TimingTowerColumn) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
    #[allow(unused_variables)]
    fn visit_timing_tower_column_folder(
        &mut self,
        folder: &mut TimingTowerColumnFolder,
    ) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
    #[allow(unused_variables)]
    fn leave_timing_tower_column_folder(
        &mut self,
        folder: &mut TimingTowerColumnFolder,
    ) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
    #[allow(unused_variables)]
    fn visit_asset(&mut self, asset: &mut AssetDefinition) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
    #[allow(unused_variables)]
    fn visit_asset_folder(&mut self, folder: &mut AssetFolder) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
    #[allow(unused_variables)]
    fn leave_asset_folder(&mut self, folder: &mut AssetFolder) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
    #[allow(unused_variables)]
    fn visit_variable(&mut self, variable: &mut VariableDefinition) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
    #[allow(unused_variables)]
    fn visit_variable_folder(&mut self, folder: &mut VariableFolder) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
    #[allow(unused_variables)]
    fn leave_variable_folder(&mut self, folder: &mut VariableFolder) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
    #[allow(unused_variables)]
    fn visit_scene(&mut self, scene: &mut SceneDefinition) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
    #[allow(unused_variables)]
    fn leave_scene(&mut self, scene: &mut SceneDefinition) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
    #[allow(unused_variables)]
    fn visit_clip_area(&mut self, clip_area: &mut dyn DynClipArea) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
    #[allow(unused_variables)]
    fn leave_clip_area(&mut self, clip_area: &mut dyn DynClipArea) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
}
