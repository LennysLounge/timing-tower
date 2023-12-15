use std::{any::Any, ops::ControlFlow};

use uuid::Uuid;

use super::{
    assets::AssetDefinition,
    folder::FolderInfo,
    scene::SceneDefinition,
    timing_tower::{TimingTower, TimingTowerColumn, TimingTowerRow, TimingTowerTable},
    variables::VariableDefinition,
    StyleDefinition,
};
pub trait StyleNode: ToAny + Visitable {
    fn id(&self) -> &Uuid;
}

pub trait ToAny {
    fn as_any(&self) -> &dyn Any;
    fn to_any(self: Box<Self>) -> Box<dyn Any>;
}
impl<T> ToAny for T
where
    T: Visitable + 'static,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn to_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

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
    fn visit_folder(&mut self, folder: &dyn FolderInfo) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
    #[allow(unused_variables)]
    fn leave_folder(&mut self, folder: &dyn FolderInfo) -> ControlFlow<()> {
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
    fn visit_timing_tower_table(&mut self, table: &TimingTowerTable) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
    #[allow(unused_variables)]
    fn leave_timing_tower_table(&mut self, table: &TimingTowerTable) -> ControlFlow<()> {
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
    fn visit_asset(&mut self, asset: &AssetDefinition) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
    #[allow(unused_variables)]
    fn visit_variable(&mut self, variable: &VariableDefinition) -> ControlFlow<()> {
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
    fn visit_folder(&mut self, folder: &mut dyn FolderInfo) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
    #[allow(unused_variables)]
    fn leave_folder(&mut self, folder: &mut dyn FolderInfo) -> ControlFlow<()> {
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
    fn visit_timing_tower_table(&mut self, table: &mut TimingTowerTable) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
    #[allow(unused_variables)]
    fn leave_timing_tower_table(&mut self, table: &mut TimingTowerTable) -> ControlFlow<()> {
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
    fn visit_asset(&mut self, asset: &mut AssetDefinition) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
    #[allow(unused_variables)]
    fn visit_variable(&mut self, variable: &mut VariableDefinition) -> ControlFlow<()> {
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
}
