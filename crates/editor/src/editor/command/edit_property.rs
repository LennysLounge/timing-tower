use std::{any::Any, ops::BitOrAssign, time::Instant};

use backend::style::StyleDefinition;
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

    pub fn execute(self, _style: &mut StyleDefinition) -> Option<EditorCommand> {
        Some(self.into())
        // SearchVisitorMut::new(self.node_id, |style_node| {
        //     let mut visitor = ApplyEditVisitor {
        //         value: Some(self.value),
        //     };
        //     style_node.enter_mut(&mut visitor);
        //     visitor.value.take().map(|v| EditProperty {
        //         timestamp: self.timestamp,
        //         node_id: self.node_id,
        //         widget_id: self.widget_id,
        //         value: v,
        //     })
        // })
        // .search_in(style)
        // .flatten()
        // .map(|c| c.into())
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

// #[derive(Clone)]
// pub struct NewValue<T> {
//     pub new_value: T,
// }

// pub trait AnyNewValue: Sync + Send + DynClone {
//     fn set(&self, subject: &mut dyn StyleNode) -> Option<Box<dyn AnyNewValue>>;
// }

// dyn_clone::clone_trait_object!(AnyNewValue);

// impl<T> AnyNewValue for NewValue<T>
// where
//     T: Send + Sync + Clone + 'static,
// {
//     fn set(&self, subject: &mut dyn StyleNode) -> Option<Box<dyn AnyNewValue>> {
//         let Some(typed_subject) = subject.as_any_mut().downcast_mut::<T>() else {
//             return None;
//         };
//         let old_value = typed_subject.clone();

//         // swap to keep self valid
//         *typed_subject = self.new_value.clone();

//         Some(Box::new(NewValue {
//             new_value: old_value,
//         }))
//     }
// }

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

// fn apply_edit<T>(dest: &mut T, src: Box<dyn Any + Sync + Send>) -> Box<dyn Any + Sync + Send>
// where
//     T: Clone + Sync + Send + 'static,
// {
//     let old_value = Box::new(dest.clone());
//     *dest = *src.downcast::<T>().expect("Cannot downcast");
//     old_value
// }

// struct ApplyEditVisitor {
//     value: Option<Box<dyn Any + Sync + Send>>,
// }
// impl NodeVisitorMut for ApplyEditVisitor {
//     fn visit_style(&mut self, style: &mut StyleDefinition) -> ControlFlow<()> {
//         self.value.take().map(|v| {
//             self.value.insert(apply_edit(style, v));
//         });
//         ControlFlow::Continue(())
//     }

//     fn visit_timing_tower(&mut self, tower: &mut TimingTower) -> ControlFlow<()> {
//         self.value.take().map(|v| {
//             self.value.insert(apply_edit(tower, v));
//         });
//         ControlFlow::Continue(())
//     }

//     fn visit_timing_tower_row(&mut self, row: &mut TimingTowerRow) -> ControlFlow<()> {
//         self.value.take().map(|v| {
//             self.value.insert(apply_edit(row, v));
//         });
//         ControlFlow::Continue(())
//     }

//     fn visit_timing_tower_column(&mut self, column: &mut TimingTowerColumn) -> ControlFlow<()> {
//         self.value.take().map(|v| {
//             self.value.insert(apply_edit(column, v));
//         });
//         ControlFlow::Continue(())
//     }

//     fn visit_timing_tower_column_folder(
//         &mut self,
//         folder: &mut TimingTowerColumnFolder,
//     ) -> ControlFlow<()> {
//         self.value.take().map(|v| {
//             self.value.insert(apply_edit(folder, v));
//         });
//         ControlFlow::Continue(())
//     }

//     fn visit_asset(&mut self, asset: &mut AssetDefinition) -> ControlFlow<()> {
//         self.value.take().map(|v| {
//             self.value.insert(apply_edit(asset, v));
//         });
//         ControlFlow::Continue(())
//     }

//     fn visit_asset_folder(&mut self, folder: &mut AssetFolder) -> ControlFlow<()> {
//         self.value.take().map(|v| {
//             self.value.insert(apply_edit(folder, v));
//         });
//         ControlFlow::Continue(())
//     }

//     fn visit_variable(&mut self, variable: &mut VariableDefinition) -> ControlFlow<()> {
//         self.value.take().map(|v| {
//             self.value.insert(apply_edit(variable, v));
//         });
//         ControlFlow::Continue(())
//     }

//     fn visit_variable_folder(&mut self, folder: &mut VariableFolder) -> ControlFlow<()> {
//         self.value.take().map(|v| {
//             self.value.insert(apply_edit(folder, v));
//         });
//         ControlFlow::Continue(())
//     }

//     fn visit_scene(&mut self, scene: &mut SceneDefinition) -> ControlFlow<()> {
//         self.value.take().map(|v| {
//             self.value.insert(apply_edit(scene, v));
//         });
//         ControlFlow::Continue(())
//     }

//     fn visit_clip_area(&mut self, clip_area: &mut dyn DynClipArea) -> ControlFlow<()> {
//         if let Some(clip_area_row) = clip_area
//             .as_any_mut()
//             .downcast_mut::<ClipArea<TimingTowerRow>>()
//         {
//             self.value.take().map(|v| {
//                 self.value.insert(apply_edit(clip_area_row, v));
//             });
//         }
//         ControlFlow::Continue(())
//     }
// }
