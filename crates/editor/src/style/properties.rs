use bevy::prelude::Color;
use bevy_egui::egui::{ComboBox, DragValue, Response, TextEdit, Ui, Widget};
use serde::{Deserialize, Serialize};

use crate::{
    reference_store::ReferenceStore,
    value_store::{
        types::{Boolean, Number, Text, Texture, Tint},
        ValueRef, ValueType,
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
    ) -> PropertyEditor<'a, T> {
        PropertyEditor {
            property,
            reference_store,
        }
    }
}
impl Widget for PropertyEditor<'_, Number> {
    fn ui(self, ui: &mut Ui) -> Response {
        match self.property {
            Property::Fixed(c) => {
                let mut res = ui.add(DragValue::new(&mut c.0));

                //TODO: The editor should also return a response.
                if let Some(reference) = self
                    .reference_store
                    .editor_small(ui, |v| v.asset_type.can_cast_to(&ValueType::Number))
                {
                    *self.property = Property::ValueRef(ValueRef {
                        id: reference.id,
                        phantom: std::marker::PhantomData,
                    });
                    res.mark_changed();
                }
                res
            }
            Property::ValueRef(value_ref) => {
                let mut changed = false;
                let res = self.reference_store.editor(ui, &value_ref.id, |v| {
                    v.asset_type.can_cast_to(&ValueType::Number)
                });

                if let Some(new_ref) = res.inner {
                    value_ref.id = new_ref.id;
                    changed = true;
                }
                let button_res = ui.button("x");
                if button_res.clicked() {
                    *self.property = Property::Fixed(Number(0.0));
                    changed = true;
                }
                let mut final_res = Response::union(&res.response, button_res);
                if changed {
                    final_res.mark_changed()
                }
                final_res
            }
        }
    }
}

impl Property<Text> {
    pub fn editor(&mut self, ui: &mut Ui, asset_repo: &ReferenceStore) -> bool {
        let mut changed = false;
        match self {
            Property::Fixed(text) => {
                changed |= ui
                    .add(TextEdit::singleline(&mut text.0).desired_width(100.0))
                    .changed();
                if let Some(reference) =
                    asset_repo.editor_small(ui, |v| v.asset_type.can_cast_to(&ValueType::Text))
                {
                    *self = Property::ValueRef(ValueRef::<Text> {
                        id: reference.id,
                        phantom: std::marker::PhantomData,
                    });
                    changed |= true;
                }
            }
            Property::ValueRef(value_ref) => {
                let new_ref = asset_repo.editor(ui, &value_ref.id, |v| {
                    v.asset_type.can_cast_to(&ValueType::Text)
                });
                if let Some(new_ref) = new_ref.inner {
                    value_ref.id = new_ref.id;
                    changed |= true;
                }
                if ui.button("x").clicked() {
                    *self = Property::Fixed(Text("".to_string()));
                    changed |= true;
                }
            }
        }
        changed
    }
}

impl Property<Number> {
    pub fn editor(&mut self, ui: &mut Ui, asset_repo: &ReferenceStore) -> bool {
        let mut changed = false;
        match self {
            Property::Fixed(c) => {
                changed |= ui.add(DragValue::new(&mut c.0)).changed();
                if let Some(reference) =
                    asset_repo.editor_small(ui, |v| v.asset_type.can_cast_to(&ValueType::Number))
                {
                    *self = Property::ValueRef(ValueRef {
                        id: reference.id,
                        phantom: std::marker::PhantomData,
                    });
                    changed = true;
                }
            }
            Property::ValueRef(value_ref) => {
                let new_ref = asset_repo.editor(ui, &value_ref.id, |v| {
                    v.asset_type.can_cast_to(&ValueType::Number)
                });
                if let Some(new_ref) = new_ref.inner {
                    value_ref.id = new_ref.id;
                    changed = true;
                }
                if ui.button("x").clicked() {
                    *self = Property::Fixed(Number(0.0));
                    changed = true;
                }
            }
        }
        changed
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

                if let Some(reference) =
                    asset_repo.editor_small(ui, |v| v.asset_type.can_cast_to(&ValueType::Tint))
                {
                    *self = Property::ValueRef(ValueRef {
                        id: reference.id,
                        phantom: std::marker::PhantomData,
                    });
                    changed |= true;
                }
            }
            Property::ValueRef(value_ref) => {
                let new_ref = asset_repo.editor(ui, &value_ref.id, |v| {
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
                if let Some(reference) =
                    asset_repo.editor_small(ui, |v| v.asset_type.can_cast_to(&ValueType::Boolean))
                {
                    *self = Property::ValueRef(ValueRef {
                        id: reference.id,
                        phantom: std::marker::PhantomData,
                    });
                    changed |= true;
                }
            }
            Property::ValueRef(value_ref) => {
                let new_ref = asset_repo.editor(ui, &value_ref.id, |v| {
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
                if let Some(reference) =
                    asset_repo.editor_none(ui, |v| v.asset_type.can_cast_to(&ValueType::Texture))
                {
                    *self = Property::ValueRef(ValueRef {
                        id: reference.id,
                        phantom: std::marker::PhantomData,
                    });
                    changed |= true;
                }
            }
            Property::ValueRef(value_ref) => {
                let new_ref = asset_repo.editor(ui, &value_ref.id, |v| {
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
