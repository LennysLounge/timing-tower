use backend::style::{StyleDefinition, StyleNode};
use dyn_clone::DynClone;
use uuid::Uuid;

use crate::style::visitors::search::SearchVisitorMut;

use super::EditorCommand;

pub struct EditProperty {
    pub id: Uuid,
    pub new_value: Box<dyn AnyNewValue>,
}
impl EditProperty {
    pub fn execute(self, style: &mut StyleDefinition) -> Option<EditorCommand> {
        SearchVisitorMut::new(self.id, |style_node| {
            self.new_value
                .set(style_node)
                .map(|new_value| EditProperty {
                    id: self.id,
                    new_value,
                })
        })
        .search_in(style)
        .flatten()
        .map(|c| c.into())
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

// /// The Setter combines an accessor function with a value.
// /// The accessor function returns a mutable reference to the field that is suposed to change.
// /// This way it is possible to defer the setting of a field without having to know what field or
// /// value to set.
// #[derive(Clone)]
// pub struct Setter<Input, Value> {
//     /// Function pointer that returns a reference to the value that is supposed to change
//     pub accessor: fn(&mut Input) -> &mut Value,
//     /// The value to set
//     pub value: Value,
// }

// /// Combines multiple `Setter` into a trait object.
// /// Setters are generic and therefore exists in many generic forms for each
// /// input and value pair.
// /// This trait makes it possible to interact with all Setters without having
// /// know their generic types.
// pub trait AnySetter: Sync + Send + DynClone {
//     /// Executes the setter on the subject and returns a new setter to undo this change.
//     /// The return value is `None` if the setter expected a different type
//     /// that the provieded subject. In this case the setter cannot set the
//     /// value and no inverse action exists.
//     fn execute_on(&self, subject: &mut dyn StyleNode) -> Option<Box<dyn AnySetter>>;
// }

// dyn_clone::clone_trait_object!(AnySetter);

// impl<Input, Value> AnySetter for Setter<Input, Value>
// where
//     Value: Clone + Sync + Send + 'static,
//     Input: Clone + 'static,
// {
//     fn execute_on(&self, subject: &mut dyn StyleNode) -> Option<Box<dyn AnySetter>> {
//         let Some(typed_subject) = subject.as_any_mut().downcast_mut::<Input>() else {
//             return None;
//         };

//         // save old value;
//         let old_value = (self.accessor)(typed_subject).clone();

//         // apply the setter
//         *(self.accessor)(typed_subject) = self.value.clone();

//         // make new setter for the undo
//         Some(Box::new(Setter {
//             accessor: self.accessor,
//             value: old_value,
//         }))
//     }
// }
