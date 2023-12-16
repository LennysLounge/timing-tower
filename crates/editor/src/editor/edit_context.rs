use std::time::Instant;

use bevy_egui::egui::{Id, Response, Ui};

use crate::editor::command::edit_property::Setter;

use super::command::{edit_property::AnySetter, EditorCommand};

struct EditPoint {
    last_edit: Instant,
    egui_id: Id,
    setter: Box<dyn AnySetter>,
}
pub struct EditContext {
    edit_point: Option<EditPoint>,
}
impl EditContext {
    pub fn new(
        ui: &mut Ui,
        mut add_content: impl FnMut(&mut EditContext),
    ) -> Option<EditorCommand> {
        // let editor_context_id = Id::new("Editor context persistant id");
        // let edit_point = ui.data_mut(|d| d.get_persisted::<EditPoint>(editor_context_id));

        let mut context = EditContext { edit_point: None };
        add_content(&mut context);
        None
    }

    pub fn edit<Input, Value>(
        &mut self,
        ui: &mut Ui,
        subject: &mut Input,
        accessor: fn(&mut Input) -> &mut Value,
    ) where
        Self: GenericEditor<Value>,
    {
        let response = self.generic_editor(ui, (accessor)(subject));
        if response.changed() {
            let setter = Setter {
                accessor,
                value: *(accessor)(subject),
            };
            self.edit_point = Some(EditPoint {
                last_edit: Instant::now(),
                egui_id: response.id,
                setter: Box::new(setter),
            });
            println!("Changed");
        }
    }
}

pub trait GenericEditor<T> {
    fn generic_editor(&mut self, ui: &mut Ui, value: &mut T) -> Response;
}

impl GenericEditor<String> for EditContext {
    fn generic_editor(&mut self, ui: &mut Ui, value: &mut String) -> Response {
        ui.text_edit_singleline(value)
    }
}
