use backend::value_types::{
    Boolean, Font, Number, Property, Text, Texture, Tint, ValueType, ValueTypeOf,
};
use bevy_egui::egui::{self, vec2, DragValue, Rect, Response, Sense, TextEdit, Ui, Widget};

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
        ui.scope(|ui| match self.property {
            Property::Fixed(c) => {
                let mut child_ui = ui.child_ui(
                    Rect::from_min_size(
                        ui.cursor().min,
                        vec2(
                            if ui.layout().horizontal_justify() {
                                ui.available_width() - 20.0
                            } else {
                                0.0
                            },
                            if ui.layout().vertical_justify() {
                                ui.available_height()
                            } else {
                                0.0
                            },
                        ),
                    ),
                    *ui.layout(),
                );
                let value_res = c.editor(&mut child_ui);
                ui.allocate_rect(child_ui.min_rect(), Sense::hover());

                let editor_res = self.reference_store.editor_small::<T>(ui);
                if let Some(new_value_ref) = editor_res.inner {
                    *self.property = Property::ValueRef(new_value_ref);
                }

                Response::union(&value_res, editor_res.response)
            }
            Property::ValueRef(value_ref) => {
                let editor_res = ui
                    .allocate_ui_at_rect(
                        Rect::from_min_size(
                            ui.cursor().min,
                            vec2(
                                if ui.layout().horizontal_justify() {
                                    ui.available_width() - 20.0
                                } else {
                                    0.0
                                },
                                if ui.layout().vertical_justify() {
                                    ui.available_height()
                                } else {
                                    0.0
                                },
                            ),
                        ),
                        |ui| self.reference_store.editor(ui, value_ref),
                    )
                    .inner;

                let mut button_res = ui.button("x");
                if button_res.clicked() {
                    *self.property = Property::Fixed(T::default());
                    button_res.mark_changed();
                }
                Response::union(&editor_res, button_res)
            }
        })
        .inner
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
        ui.add(egui::Checkbox::new(&mut self.0, ""))
    }
}

impl ValueTypeEditor for Texture {
    fn editor(&mut self, ui: &mut Ui) -> Response {
        ui.label("None")
    }
}
impl ValueTypeEditor for Font {
    fn editor(&mut self, ui: &mut Ui) -> Response {
        ui.label("Default")
    }
}
