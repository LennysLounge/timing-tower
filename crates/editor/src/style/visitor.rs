use backend::style::{
    assets::AssetDefinition,
    folder::{Folder, FolderOrT},
    timing_tower::{TimingTower, TimingTowerColumn, TimingTowerRow, TimingTowerTable},
    variables::VariableDefinition,
    StyleDefinition,
};
use uuid::Uuid;

pub trait VisitableStyle {
    fn accept(&mut self, visitor: &mut dyn StyleVisitor) -> bool;
}
pub trait StyleVisitor {
    fn visit_style(&mut self, style: &mut StyleDefinition) -> bool {
        _ = style;
        true
    }
    fn leave_style(&mut self, style: &mut StyleDefinition) -> bool {
        _ = style;
        true
    }
    fn visit_asset(&mut self, asset: &mut AssetDefinition) -> bool {
        _ = asset;
        true
    }
    fn visit_variable(&mut self, variable: &mut VariableDefinition) -> bool {
        _ = variable;
        true
    }
    fn visit_timing_tower(&mut self, timing_tower: &mut TimingTower) -> bool {
        _ = timing_tower;
        true
    }
    fn leave_timing_tower(&mut self, timing_tower: &mut TimingTower) -> bool {
        _ = timing_tower;
        true
    }
    fn visit_timing_tower_table(&mut self, table: &mut TimingTowerTable) -> bool {
        _ = table;
        true
    }
    fn leave_timing_tower_table(&mut self, table: &mut TimingTowerTable) -> bool {
        _ = table;
        true
    }
    fn visit_timing_tower_row(&mut self, row: &mut TimingTowerRow) -> bool {
        _ = row;
        true
    }
    fn leave_timing_tower_row(&mut self, row: &mut TimingTowerRow) -> bool {
        _ = row;
        true
    }
    fn visit_timing_tower_column(&mut self, column: &mut TimingTowerColumn) -> bool {
        _ = column;
        true
    }
    fn visit_folder(&mut self, folder: AnyFolder) -> bool {
        _ = folder;
        true
    }
    fn leave_folder(&mut self, folder: AnyFolder) -> bool {
        _ = folder;
        true
    }
}

pub enum AnyFolder<'a> {
    Asset(&'a mut Folder<AssetDefinition>),
    Variable(&'a mut Folder<VariableDefinition>),
    Column(&'a mut Folder<TimingTowerColumn>),
}
impl AnyFolder<'_> {
    pub fn id(&self) -> Uuid {
        match self {
            AnyFolder::Asset(f) => f.id,
            AnyFolder::Variable(f) => f.id,
            AnyFolder::Column(f) => f.id,
        }
    }
    pub fn name(&self) -> &str {
        match self {
            AnyFolder::Asset(f) => &f.name,
            AnyFolder::Variable(f) => &f.name,
            AnyFolder::Column(f) => &f.name,
        }
    }
}

impl VisitableStyle for StyleDefinition {
    fn accept(&mut self, visitor: &mut dyn StyleVisitor) -> bool {
        if visitor.visit_style(self) {
            self.assets.accept(visitor);
            self.vars.accept(visitor);
            self.timing_tower.accept(visitor);
        }
        visitor.leave_style(self)
    }
}

impl<T> VisitableStyle for FolderOrT<T>
where
    T: VisitableStyle,
    Folder<T>: VisitableStyle,
{
    fn accept(&mut self, visitor: &mut dyn StyleVisitor) -> bool {
        match self {
            FolderOrT::T(t) => t.accept(visitor),
            FolderOrT::Folder(f) => f.accept(visitor),
        }
    }
}

impl VisitableStyle for Folder<AssetDefinition> {
    fn accept(&mut self, visitor: &mut dyn StyleVisitor) -> bool {
        if visitor.visit_folder(AnyFolder::Asset(self)) {
            self.content.iter_mut().all(|f| f.accept(visitor));
        }
        visitor.leave_folder(AnyFolder::Asset(self))
    }
}

impl VisitableStyle for AssetDefinition {
    fn accept(&mut self, visitor: &mut dyn StyleVisitor) -> bool {
        visitor.visit_asset(self)
    }
}

impl VisitableStyle for Folder<VariableDefinition> {
    fn accept(&mut self, visitor: &mut dyn StyleVisitor) -> bool {
        if visitor.visit_folder(AnyFolder::Variable(self)) {
            self.content.iter_mut().all(|f| f.accept(visitor));
        }
        visitor.leave_folder(AnyFolder::Variable(self))
    }
}

impl VisitableStyle for VariableDefinition {
    fn accept(&mut self, visitor: &mut dyn StyleVisitor) -> bool {
        visitor.visit_variable(self)
    }
}

impl VisitableStyle for TimingTower {
    fn accept(&mut self, visitor: &mut dyn StyleVisitor) -> bool {
        if visitor.visit_timing_tower(self) {
            self.table.accept(visitor);
        }
        visitor.leave_timing_tower(self)
    }
}

impl VisitableStyle for TimingTowerTable {
    fn accept(&mut self, visitor: &mut dyn StyleVisitor) -> bool {
        if visitor.visit_timing_tower_table(self) {
            self.row.accept(visitor);
        }
        visitor.leave_timing_tower_table(self)
    }
}

impl VisitableStyle for TimingTowerRow {
    fn accept(&mut self, visitor: &mut dyn StyleVisitor) -> bool {
        if visitor.visit_timing_tower_row(self) {
            self.columns.accept(visitor);
        }
        visitor.leave_timing_tower_row(self)
    }
}

impl VisitableStyle for Folder<TimingTowerColumn> {
    fn accept(&mut self, visitor: &mut dyn StyleVisitor) -> bool {
        if visitor.visit_folder(AnyFolder::Column(self)) {
            self.content.iter_mut().all(|f| f.accept(visitor));
        }
        visitor.leave_folder(AnyFolder::Column(self))
    }
}

impl VisitableStyle for TimingTowerColumn {
    fn accept(&mut self, visitor: &mut dyn StyleVisitor) -> bool {
        visitor.visit_timing_tower_column(self)
    }
}
