use bevy_egui::egui::Ui;
use uuid::Uuid;

use crate::{
    asset_repo::{AssetId, AssetReference, VariableDefinition},
    style::{
        folder::{Folder, FolderOrT},
        variables::VariableBehavior,
    },
};

pub struct AssetReferenceRepo {
    assets: Vec<AssetOrFolder>,
}

impl AssetReferenceRepo {
    pub fn new(vars: &Folder<VariableBehavior>) -> Self {
        let assets = match AssetOrFolder::from(vars) {
            AssetOrFolder::Asset(_) => unreachable!(),
            AssetOrFolder::Folder { assets, .. } => assets,
        };
        Self { assets }
    }

    pub fn editor(
        &self,
        ui: &mut Ui,
        asset_ref: &AssetReference,
        is_type_allowed: impl Fn(&AssetId) -> bool,
    ) -> Option<AssetReference> {
        let button_name = self
            .get(&asset_ref.key)
            .map(|id| id.name.as_str())
            .unwrap_or("Ref");

        let mut selected_asset = None;
        ui.menu_button(button_name, |ui| {
            self.show_menu(ui, &mut selected_asset, &is_type_allowed);
        });
        selected_asset.map(|a| a.get_ref())
    }

    pub fn editor_small(
        &self,
        ui: &mut Ui,
        is_type_allowed: impl Fn(&AssetId) -> bool,
    ) -> Option<AssetReference> {
        let mut selected_asset = None;
        ui.menu_button("R", |ui| {
            self.show_menu(ui, &mut selected_asset, &is_type_allowed);
        });
        selected_asset.map(|a| a.get_ref())
    }

    fn get(&self, id: &Uuid) -> Option<&AssetId> {
        self.assets.iter().find_map(|a| a.get(id))
    }

    fn show_menu(
        &self,
        ui: &mut Ui,
        selected_asset: &mut Option<AssetId>,
        is_type_allowed: &impl Fn(&AssetId) -> bool,
    ) {
        for a in self.assets.iter() {
            a.show_menu(ui, selected_asset, is_type_allowed);
        }
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
    fn from(vars: &Folder<VariableBehavior>) -> Self {
        Self::Folder {
            name: vars.name.clone(),
            assets: vars
                .content
                .iter()
                .map(|c| match c {
                    FolderOrT::T(t) => Self::Asset(t.get_variable_id().clone()),
                    FolderOrT::Folder(f) => Self::from(f),
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
                ui.collapsing(name, |ui| {
                    for a in assets.iter() {
                        a.show_menu(ui, selected_asset, is_type_allowed);
                    }
                });
            }
        }
    }
}
