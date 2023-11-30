use bevy::prelude::Color;
use bevy_egui::egui::{ComboBox, DragValue, Response, TextEdit, Ui, Widget};
use serde::{Deserialize, Serialize};

use crate::{
    reference_store::ReferenceStore,
    value_store::{
        types::{Boolean, Number, Text, Texture, Tint},
        ToTypedValueRef, ToUntypedValueRef, UntypedValueRef, ValueRef, ValueType, ValueTypeOf,
    },
};

#[derive(Serialize, Deserialize, Clone)]
pub enum Property<T> {
    ValueRef(ValueRef<T>),
    #[serde(untagged)]
    Fixed(T),
}

impl<T: Default> Default for Property<T> {
    fn default() -> Self {
        Property::Fixed(T::default())
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Vec2Property {
    pub x: Property<Number>,
    pub y: Property<Number>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Vec3Property {
    pub x: Property<Number>,
    pub y: Property<Number>,
    pub z: Property<Number>,
}

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
        UntypedValueRef: ToTypedValueRef<T>,
        ValueType: ValueTypeOf<T>,
        ValueRef<T>: ToUntypedValueRef<T>,
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
    UntypedValueRef: ToTypedValueRef<T>,
    ValueType: ValueTypeOf<T>,
    ValueRef<T>: ToUntypedValueRef<T>,
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
impl Property<Tint> {
    pub fn editor(&mut self, ui: &mut Ui, asset_repo: &ReferenceStore) -> bool {
        let mut changed = false;
        match self {
            Property::Fixed(c) => {
                let mut color = c.0.as_rgba_f32();
                changed |= ui.color_edit_button_rgba_unmultiplied(&mut color).changed();
                c.0 = color.into();

                if let Some(reference) = asset_repo
                    .untyped_editor_small(ui, |v| v.asset_type.can_cast_to(&ValueType::Tint))
                    .inner
                {
                    *self = Property::ValueRef(ValueRef {
                        id: reference.id,
                        phantom: std::marker::PhantomData,
                    });
                    changed |= true;
                }
            }
            Property::ValueRef(value_ref) => {
                let new_ref = asset_repo.untyped_editor(ui, &value_ref.id, |v| {
                    v.asset_type.can_cast_to(&ValueType::Tint)
                });
                if let Some(new_ref) = new_ref.inner {
                    value_ref.id = new_ref.id;
                    changed |= true;
                }
                if ui.button("x").clicked() {
                    *self = Property::Fixed(Tint(Color::PURPLE));
                    changed |= true;
                }
            }
        }
        changed
    }
}

impl Property<Boolean> {
    pub fn editor(&mut self, ui: &mut Ui, asset_repo: &ReferenceStore) -> bool {
        let mut changed = false;
        match self {
            Property::Fixed(b) => {
                ComboBox::from_id_source(ui.next_auto_id())
                    .width(50.0)
                    .selected_text(match b.0 {
                        true => "Yes",
                        false => "No",
                    })
                    .show_ui(ui, |ui| {
                        changed |= ui.selectable_value(&mut b.0, true, "Yes").changed();
                        changed |= ui.selectable_value(&mut b.0, false, "No").changed();
                    });
                if let Some(reference) = asset_repo
                    .untyped_editor_small(ui, |v| v.asset_type.can_cast_to(&ValueType::Boolean))
                    .inner
                {
                    *self = Property::ValueRef(ValueRef {
                        id: reference.id,
                        phantom: std::marker::PhantomData,
                    });
                    changed |= true;
                }
            }
            Property::ValueRef(value_ref) => {
                let new_ref = asset_repo.untyped_editor(ui, &value_ref.id, |v| {
                    v.asset_type.can_cast_to(&ValueType::Tint)
                });
                if let Some(new_ref) = new_ref.inner {
                    value_ref.id = new_ref.id;
                    changed |= true;
                }
                if ui.button("x").clicked() {
                    *self = Property::Fixed(Boolean(true));
                    changed |= true;
                }
            }
        }
        changed
    }
}

impl Property<Texture> {
    pub fn editor(&mut self, ui: &mut Ui, asset_repo: &ReferenceStore) -> bool {
        let mut changed = false;
        match self {
            Property::Fixed(..) => {
                if let Some(reference) = asset_repo
                    .untyped_editor_none(ui, |v| v.asset_type.can_cast_to(&ValueType::Texture))
                {
                    *self = Property::ValueRef(ValueRef {
                        id: reference.id,
                        phantom: std::marker::PhantomData,
                    });
                    changed |= true;
                }
            }
            Property::ValueRef(value_ref) => {
                let new_ref = asset_repo.untyped_editor(ui, &value_ref.id, |v| {
                    v.asset_type.can_cast_to(&ValueType::Texture)
                });
                if let Some(new_ref) = new_ref.inner {
                    value_ref.id = new_ref.id;
                    changed |= true;
                }
                if ui.button("x").clicked() {
                    *self = Property::Fixed(Texture::None);
                    changed |= true;
                }
            }
        }
        changed
    }
}
