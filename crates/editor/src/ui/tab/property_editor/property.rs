use backend::value_types::{Boolean, Font, Number, Property, Text, Texture, Tint, Value};
use bevy_egui::egui::{
    self, vec2, DragValue, InnerResponse, NumExt, Rect, Response, TextEdit, Ui, Widget,
};

use crate::reference_store::{producer_id_editor, select_producer_reference, ReferenceStore};

pub struct PropertyEditor<'a, T> {
    property: &'a mut Property<T>,
    reference_store: &'a ReferenceStore,
}
impl<'a, T> PropertyEditor<'a, T>
where
    T: Value + Default + ValueTypeEditor,
{
    pub fn new(
        property: &'a mut Property<T>,
        reference_store: &'a ReferenceStore,
    ) -> PropertyEditor<'a, T> {
        PropertyEditor {
            property,
            reference_store,
        }
    }

    fn left_ui(&mut self, ui: &mut Ui) -> Response {
        match self.property {
            Property::Producer(producer_id) => {
                producer_id_editor(ui, self.reference_store, producer_id, |v| {
                    v.value_type.can_cast_to(&T::ty())
                })
            }
            Property::Fixed(value) => value.editor(ui),
        }
    }

    fn right_ui(&mut self, ui: &mut Ui) -> Response {
        match self.property {
            Property::Producer(_) => {
                let mut button_res = ui.button("x");
                if button_res.clicked() {
                    *self.property = Property::Fixed(T::default());
                    button_res.mark_changed();
                }
                button_res
            }
            Property::Fixed(_) => {
                let InnerResponse {
                    inner: selected_producer,
                    mut response,
                } = select_producer_reference(ui, self.reference_store, "R", |v| {
                    v.value_type.can_cast_to(&T::ty())
                });
                if let Some(selected_producer) = selected_producer {
                    *self.property = Property::Producer(selected_producer.id);
                    response.mark_changed();
                }
                response
            }
        }
    }
}
impl<T> Widget for PropertyEditor<'_, T>
where
    T: Value + Default + ValueTypeEditor,
{
    fn ui(mut self, ui: &mut Ui) -> Response {
        let res = ui.scope(|ui| {
            // This gets us the justified size for this widget. `Ui::allocate_space`
            // will increase the requested size if the ui layout is justified. This
            // way we get the justified size without calculating it manually.
            let (_id, base_rect) = ui.allocate_space(vec2(0.0, 0.0));

            // Leave some space on the right for the Ref button.
            let left = base_rect.with_max_x(
                (base_rect.max.x - 18.0 - ui.spacing().item_spacing.x).at_least(base_rect.min.x),
            );
            let InnerResponse {
                inner: left_has_changed,
                response: left_res,
            } = ui.allocate_ui_at_rect(left, |ui| {
                ui.centered_and_justified(|ui| self.left_ui(ui))
                    .inner
                    .changed()
            });

            // The right side is just to the left of the left size plus the item spaceing.
            // Right side has a fixed size.
            let right = Rect::from_min_size(
                left_res.rect.right_top() + vec2(ui.spacing().item_spacing.x, 0.0),
                vec2(18.0, left_res.rect.height()),
            );
            let InnerResponse {
                inner: right_has_changed,
                response: _,
            } = ui.allocate_ui_at_rect(right, |ui| {
                ui.centered_and_justified(|ui| self.right_ui(ui))
                    .inner
                    .changed()
            });

            left_has_changed || right_has_changed
        });

        let InnerResponse {
            inner: has_changed,
            mut response,
        } = res;
        if has_changed {
            response.mark_changed();
        }
        response
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
