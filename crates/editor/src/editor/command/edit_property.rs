use std::{any::Any, ops::BitOrAssign, time::Instant};

use backend::style::{
    visitor::{NodeIterator, NodeMut},
    StyleDefinition,
};
use bevy_egui::egui::{self, Response};
use uuid::Uuid;

use super::EditorCommand;

pub struct EditProperty {
    pub timestamp: Instant,
    pub node_id: Uuid,
    pub widget_id: egui::Id,
    pub value: Box<dyn Any + Sync + Send>,
}
impl EditProperty {
    pub fn new<T>(node_id: Uuid, new_value: T, widget_id: egui::Id) -> Self
    where
        T: Sync + Send + Clone + 'static,
    {
        Self {
            timestamp: Instant::now(),
            node_id,
            widget_id,
            value: Box::new(new_value),
        }
    }

    pub fn execute(self, style: &mut StyleDefinition) -> Option<EditorCommand> {
        style
            .search_mut(&self.node_id, |node| {
                let old_value = match node {
                    NodeMut::Style(o) => apply_edit(o, self.value),
                    NodeMut::Variable(o) => apply_edit(o, self.value),
                    NodeMut::VariableFolder(o) => apply_edit(o, self.value),
                    NodeMut::Asset(o) => apply_edit(o, self.value),
                    NodeMut::AssetFolder(o) => apply_edit(o, self.value),
                    NodeMut::Scene(o) => apply_edit(o, self.value),
                    NodeMut::TimingTower(o) => apply_edit(o, self.value),
                    NodeMut::TimingTowerRow(o) => apply_edit(o, self.value),
                    NodeMut::TimingTowerColumn(o) => apply_edit(o, self.value),
                    NodeMut::TimingTowerColumnFolder(o) => apply_edit(o, self.value),
                    NodeMut::ClipArea(o) => apply_edit(o, self.value),
                };
                EditProperty {
                    timestamp: self.timestamp,
                    node_id: self.node_id,
                    widget_id: self.widget_id,
                    value: old_value,
                }
            })
            .map(|c| c.into())
    }

    pub fn can_merge_with(&self, other: &EditProperty) -> bool {
        self.node_id == other.node_id
            && self.widget_id == other.widget_id
            && other.timestamp.duration_since(self.timestamp).as_secs() < 1
    }

    pub fn merge(self, other: EditProperty) -> EditProperty {
        Self {
            timestamp: other.timestamp,
            node_id: self.node_id,
            widget_id: self.widget_id,
            value: self.value,
        }
    }
}
impl From<EditProperty> for EditorCommand {
    fn from(value: EditProperty) -> Self {
        Self::EditProperty(value)
    }
}

/// The result of a undo/redo context.
pub enum EditResult {
    /// No value were changed.
    None,
    /// The value was changed by a widget with this id.
    FromId(bevy_egui::egui::Id),
}
impl BitOrAssign for EditResult {
    fn bitor_assign(&mut self, rhs: Self) {
        match rhs {
            EditResult::None => (),
            EditResult::FromId(_) => *self = rhs,
        }
    }
}
impl From<Response> for EditResult {
    fn from(value: Response) -> Self {
        if value.changed() {
            Self::FromId(value.id)
        } else {
            Self::None
        }
    }
}

fn apply_edit<T>(dest: &mut T, src: Box<dyn Any + Sync + Send>) -> Box<dyn Any + Sync + Send>
where
    T: Clone + Sync + Send + 'static,
{
    let old_value = Box::new(dest.clone());
    *dest = *src.downcast::<T>().expect("Cannot downcast");
    old_value
}
