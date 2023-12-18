use std::{ops::BitOrAssign, time::Instant};

use backend::style::{StyleDefinition, StyleNode};
use bevy_egui::egui::{self, Response};
use dyn_clone::DynClone;
use uuid::Uuid;

use crate::style::visitors::search::SearchVisitorMut;

use super::EditorCommand;

pub struct EditProperty {
    pub timestamp: Instant,
    pub node_id: Uuid,
    pub widget_id: egui::Id,
    pub new_value: Box<dyn AnyNewValue>,
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
            new_value: Box::new(NewValue { new_value }),
        }
    }

    pub fn execute(self, style: &mut StyleDefinition) -> Option<EditorCommand> {
        SearchVisitorMut::new(self.node_id, |style_node| {
            self.new_value
                .set(style_node)
                .map(|new_value| EditProperty {
                    timestamp: self.timestamp,
                    node_id: self.node_id,
                    new_value,
                    widget_id: self.widget_id,
                })
        })
        .search_in(style)
        .flatten()
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
            new_value: self.new_value,
        }
    }
}
impl From<EditProperty> for EditorCommand {
    fn from(value: EditProperty) -> Self {
        Self::EditProperty(value)
    }
}

#[derive(Clone)]
pub struct NewValue<T> {
    pub new_value: T,
}

pub trait AnyNewValue: Sync + Send + DynClone {
    fn set(&self, subject: &mut dyn StyleNode) -> Option<Box<dyn AnyNewValue>>;
}

dyn_clone::clone_trait_object!(AnyNewValue);

impl<T> AnyNewValue for NewValue<T>
where
    T: Send + Sync + Clone + 'static,
{
    fn set(&self, subject: &mut dyn StyleNode) -> Option<Box<dyn AnyNewValue>> {
        let Some(typed_subject) = subject.as_any_mut().downcast_mut::<T>() else {
            return None;
        };
        let old_value = typed_subject.clone();

        // swap to keep self valid
        *typed_subject = self.new_value.clone();

        Some(Box::new(NewValue {
            new_value: old_value,
        }))
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
