use bevy_egui::egui::{InnerResponse, Response, Ui};
use uuid::Uuid;

use crate::{
    game_sources::{self, GameSource},
    style::{
        assets::AssetDefinition,
        folder::{Folder, FolderOrT},
        variables::VariableBehavior,
    },
    value_store::{AssetId, IntoValueProducer, UntypedValueRef, ValueRef},
    value_types::{ValueType, ValueTypeOf},
};

pub struct ReferenceStore {
    assets: AssetOrFolder,
    game_sources: AssetOrFolder,
    variables: AssetOrFolder,
}

impl ReferenceStore {
    pub fn new(vars: &Folder<VariableBehavior>, assets: &Folder<AssetDefinition>) -> Self {
        Self {
            variables: AssetOrFolder::from_vars(vars),
            game_sources: AssetOrFolder::from_game(game_sources::get_game_sources()),
            assets: AssetOrFolder::from_asset_defs(assets),
        }
    }

    pub fn editor<T>(&self, ui: &mut Ui, value_ref: &mut ValueRef<T>) -> Response
    where
        ValueType: ValueTypeOf<T>,
    {
        let target_type: ValueType = ValueTypeOf::<T>::get();

        let mut editor_res = self.untyped_editor(ui, &value_ref.id, |v| {
            v.asset_type.can_cast_to(&target_type)
        });
        if let Some(UntypedValueRef { id, value_type }) = editor_res.inner {
            if !value_type.can_cast_to(&target_type) {
                unreachable!(
                    "Could not cast untyped value ref to 
                    type {} even though the ref is limited to only that type",
                    std::any::type_name::<T>()
                );
            }
            *value_ref = ValueRef {
                id: id,
                phantom: std::marker::PhantomData,
            };
            editor_res.response.mark_changed();
        }
        editor_res.response
    }

    pub fn editor_small<T>(&self, ui: &mut Ui) -> InnerResponse<Option<ValueRef<T>>>
    where
        ValueType: ValueTypeOf<T>,
    {
        let target_type: ValueType = ValueTypeOf::<T>::get();

        let mut editor_res =
            self.untyped_editor_small(ui, |v| v.asset_type.can_cast_to(&target_type));

        let inner = editor_res.inner.map(|new_untyped_value_ref| {
            if !new_untyped_value_ref.value_type.can_cast_to(&target_type) {
                unreachable!(
                    "Could not cast untyped value ref to 
                    type {} even though the ref is limited to only that type",
                    std::any::type_name::<T>()
                );
            }
            editor_res.response.mark_changed();
            ValueRef {
                id: new_untyped_value_ref.id,
                phantom: std::marker::PhantomData,
            }
        });
        InnerResponse::new(inner, editor_res.response)
    }

    pub fn untyped_editor(
        &self,
        ui: &mut Ui,
        asset_ref_key: &Uuid,
        is_type_allowed: impl Fn(&AssetId) -> bool,
    ) -> InnerResponse<Option<UntypedValueRef>> {
        let button_name = self
            .get(asset_ref_key)
            .map(|id| id.name.as_str())
            .unwrap_or("- Invalud Ref -");

        let mut selected_asset = None;
        let res = ui.menu_button(button_name, |ui| {
            self.show_menu(ui, &mut selected_asset, &is_type_allowed);
        });

        InnerResponse::new(selected_asset.map(|a| a.get_ref()), res.response)
    }

    pub fn untyped_editor_small(
        &self,
        ui: &mut Ui,
        is_type_allowed: impl Fn(&AssetId) -> bool,
    ) -> InnerResponse<Option<UntypedValueRef>> {
        let mut selected_asset = None;
        let res = ui.menu_button("R", |ui| {
            self.show_menu(ui, &mut selected_asset, &is_type_allowed);
        });
        InnerResponse::new(selected_asset.map(|a| a.get_ref()), res.response)
    }

    fn get(&self, id: &Uuid) -> Option<&AssetId> {
        self.variables
            .get(id)
            .or_else(|| self.game_sources.get(id))
            .or_else(|| self.assets.get(id))
    }

    fn show_menu(
        &self,
        ui: &mut Ui,
        selected_asset: &mut Option<AssetId>,
        is_type_allowed: &impl Fn(&AssetId) -> bool,
    ) {
        self.assets.show_menu(ui, selected_asset, is_type_allowed);
        self.game_sources
            .show_menu(ui, selected_asset, is_type_allowed);
        self.variables
            .show_menu(ui, selected_asset, is_type_allowed);
    }
}

enum AssetOrFolder {
    Asset(AssetId),
    Folder {
        name: String,
        assets: Vec<AssetOrFolder>,
    },
}
impl AssetOrFolder {
    fn from_vars(vars: &Folder<VariableBehavior>) -> Self {
        Self::Folder {
            name: vars.name.clone(),
            assets: vars
                .content
                .iter()
                .map(|c| match c {
                    FolderOrT::T(t) => Self::Asset(t.asset_id().clone()),
                    FolderOrT::Folder(f) => Self::from_vars(f),
                })
                .collect(),
        }
    }
    fn from_game(game_sources: Vec<&GameSource>) -> Self {
        Self::Folder {
            name: "Game".to_string(),
            assets: game_sources
                .into_iter()
                .map(|s| Self::Asset(s.asset_id().clone()))
                .collect(),
        }
    }
    fn from_asset_defs(assets: &Folder<AssetDefinition>) -> Self {
        Self::Folder {
            name: assets.name.clone(),
            assets: assets
                .content
                .iter()
                .map(|a| match a {
                    FolderOrT::T(def) => Self::Asset(def.asset_id().clone()),
                    FolderOrT::Folder(f) => Self::from_asset_defs(f),
                })
                .collect(),
        }
    }
    fn get(&self, id: &Uuid) -> Option<&AssetId> {
        match self {
            AssetOrFolder::Asset(asset_id) => (&asset_id.id == id).then_some(asset_id),
            AssetOrFolder::Folder { name: _, assets } => assets.iter().find_map(|a| a.get(id)),
        }
    }

    fn get_all_contained_ids(&self) -> Vec<&AssetId> {
        match self {
            AssetOrFolder::Asset(id) => vec![id],
            AssetOrFolder::Folder { name: _, assets } => assets
                .iter()
                .flat_map(|a| a.get_all_contained_ids())
                .collect(),
        }
    }

    fn show_menu(
        &self,
        ui: &mut Ui,
        selected_asset: &mut Option<AssetId>,
        is_type_allowed: &impl Fn(&AssetId) -> bool,
    ) {
        match self {
            AssetOrFolder::Asset(asset_id) => {
                let is_asset_allowed = is_type_allowed(&asset_id);
                let res = ui.add_enabled_ui(is_asset_allowed, |ui| {
                    if ui.button(&asset_id.name).clicked() {
                        *selected_asset = Some(asset_id.clone());
                        ui.close_menu();
                    }
                });
                if !is_asset_allowed {
                    res.response.on_hover_text(format!(
                        "{} type not allowed for this reference.",
                        asset_id.asset_type.name()
                    ));
                }
            }
            AssetOrFolder::Folder { name, assets } => {
                let is_any_allowed = self
                    .get_all_contained_ids()
                    .into_iter()
                    .any(|a| is_type_allowed(a));

                let res = ui.add_enabled_ui(is_any_allowed, |ui| {
                    ui.collapsing(name, |ui| {
                        for a in assets.iter() {
                            a.show_menu(ui, selected_asset, is_type_allowed);
                        }
                    });
                });
                if !is_any_allowed {
                    res.response
                        .on_hover_text("Folder contains no allowed types for this reference");
                }
            }
        }
    }
}
