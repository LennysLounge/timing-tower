use bevy_egui::egui::Ui;
use uuid::Uuid;

use crate::{
    asset_repo::{AssetDefinition, AssetId, AssetReference},
    game_sources::{self, GameSource},
    style::{
        folder::{Folder, FolderOrT},
        variables::VariableBehavior,
    },
};

pub struct AssetReferenceRepo {
    game_sources: AssetOrFolder,
    variables: AssetOrFolder,
}

impl AssetReferenceRepo {
    pub fn new(vars: &Folder<VariableBehavior>) -> Self {
        Self {
            variables: AssetOrFolder::from_vars(vars),
            game_sources: AssetOrFolder::from_game(game_sources::get_game_sources()),
        }
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
        self.variables.get(id).or_else(|| self.game_sources.get(id))
    }

    fn show_menu(
        &self,
        ui: &mut Ui,
        selected_asset: &mut Option<AssetId>,
        is_type_allowed: &impl Fn(&AssetId) -> bool,
    ) {
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
