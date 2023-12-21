use backend::value_types::{
    Boolean, Number, Property, Text, Texture, Tint, ValueType, ValueTypeOf,
};
use bevy_egui::egui::{ComboBox, DragValue, Response, TextEdit, Ui, Widget};

use crate::reference_store::ReferenceStore;

pub struct PropertyEditor<'a, T> {
    property: &'a mut Property<T>,
    reference_store: &'a ReferenceStore,
}
impl<'a, T> PropertyEditor<'a, T> {
    pub fn new(
        property: &'a mut Property<T>,
        reference_store: &'a ReferenceStore,
    ) -> PropertyEditor<'a, T>
    where
        ValueType: ValueTypeOf<T>,
        T: Default + ValueTypeEditor,
    {
        PropertyEditor {
            property,
            reference_store,
        }
    }
}

impl<T> Widget for PropertyEditor<'_, T>
where
    ValueType: ValueTypeOf<T>,
    T: Default + ValueTypeEditor,
{
    fn ui(self, ui: &mut Ui) -> Response {
        match self.property {
            Property::Fixed(c) => {
                let value_res = c.editor(ui);

                let editor_res = self.reference_store.editor_small::<T>(ui);
                if let Some(new_value_ref) = editor_res.inner {
                    *self.property = Property::ValueRef(new_value_ref);
                }

                Response::union(&value_res, editor_res.response)
            }
            Property::ValueRef(value_ref) => {
                let editor_res = self.reference_store.editor(ui, value_ref);

                let mut button_res = ui.button("x");
                if button_res.clicked() {
                    *self.property = Property::Fixed(T::default());
                    button_res.mark_changed();
                }
                Response::union(&editor_res, button_res)
            }
        }
    }
}

pub trait ValueTypeEditor {
    fn editor(&mut self, ui: &mut Ui) -> Response;
}
impl ValueTypeEditor for Number {
    fn editor(&mut self, ui: &mut Ui) -> Response {
        ui.add(DragValue::new(&mut self.0))
    }
}
impl ValueTypeEditor for Text {
    fn editor(&mut self, ui: &mut Ui) -> Response {
        ui.add(TextEdit::singleline(&mut self.0).desired_width(100.0))
    }
}
impl ValueTypeEditor for Tint {
    fn editor(&mut self, ui: &mut Ui) -> Response {
        let mut color = self.0.as_rgba_f32();
        let res = ui.color_edit_button_rgba_unmultiplied(&mut color);
        self.0 = color.into();
        res
    }
}
impl ValueTypeEditor for Boolean {
    fn editor(&mut self, ui: &mut Ui) -> Response {
        let mut changed = false;
        let mut res = ComboBox::from_id_source(ui.next_auto_id())
            .width(50.0)
            .selected_text(match self.0 {
                true => "Yes",
                false => "No",
            })
            .show_ui(ui, |ui| {
                changed |= ui.selectable_value(&mut self.0, true, "Yes").changed();
                changed |= ui.selectable_value(&mut self.0, false, "No").changed();
            });
        if changed {
            res.response.mark_changed();
        }
        res.response
    }
}

impl ValueTypeEditor for Texture {
    fn editor(&mut self, ui: &mut Ui) -> Response {
        ui.label("None")
    }
}
