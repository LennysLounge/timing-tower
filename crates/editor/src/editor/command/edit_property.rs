use backend::style::{visitor::StyleNode, StyleDefinition};
use uuid::Uuid;

use crate::style::visitors::search::SearchVisitorMut;

use super::EditorCommand;

pub struct EditProperty {
    pub id: Uuid,
    pub setter: Box<dyn AnySetter>,
}
impl EditProperty {
    pub fn execute(self, style: &mut StyleDefinition) -> Option<EditorCommand> {
        SearchVisitorMut::new(self.id, |style_node| {
            self.setter
                .execute_on(style_node)
                .map(|undo_setter| EditProperty {
                    id: self.id,
                    setter: undo_setter,
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

/// The Setter combines an accessor function with a value.
/// The accessor function returns a mutable reference to the field that is suposed to change.
/// This way it is possible to defer the setting of a field without having to know what field or
/// value to set.
pub struct Setter<Input, Value> {
    /// Function pointer that returns a reference to the value that is supposed to change
    pub accessor: fn(&mut Input) -> &mut Value,
    /// The value to set
    pub value: Value,
}

/// Combines multiple `Setter` into a trait object.
/// Setters are generic and therefore exists in many generic forms for each
/// input and value pair.
/// This trait makes it possible to interact with all Setters without having
/// know their generic types.
pub trait AnySetter: Sync + Send {
    /// Executes the setter on the subject and returns a new setter to undo this change.
    /// The return value is `None` if the setter expected a different type
    /// that the provieded subject. In this case the setter cannot set the
    /// value and no inverse action exists.
    fn execute_on(&self, subject: &mut dyn StyleNode) -> Option<Box<dyn AnySetter>>;
}

impl<Input, Value> AnySetter for Setter<Input, Value>
where
    Value: Clone + Sync + Send + 'static,
    Input: 'static,
{
    fn execute_on(&self, subject: &mut dyn StyleNode) -> Option<Box<dyn AnySetter>> {
        let Some(typed_subject) = subject.as_any_mut().downcast_mut::<Input>() else {
            return None;
        };

        // save old value;
        let old_value = (self.accessor)(typed_subject).clone();

        // apply the setter
        *(self.accessor)(typed_subject) = self.value.clone();

        // make new setter for the undo
        Some(Box::new(Setter {
            accessor: self.accessor,
            value: old_value,
        }))
    }
}
