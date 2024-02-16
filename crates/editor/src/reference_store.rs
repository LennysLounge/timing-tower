use bevy::{
    app::{First, Plugin},
    ecs::{
        event::EventReader,
        system::{Res, ResMut, Resource},
    },
};
use bevy_egui::egui::{InnerResponse, Response, Ui};
use serde::{Deserialize, Serialize};
use tracing::info;

use backend::{
    game_sources,
    savefile::{Savefile, SavefileChanged},
    style::{assets::AssetFolder, variables::VariableFolder},
    value_store::ProducerId,
    value_types::AnyProducerRef,
};

pub struct ReferenceStorePlugin;
impl Plugin for ReferenceStorePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(ReferenceStore::default())
            .add_systems(First, savefile_changed);
    }
}

pub trait IntoProducerData {
    fn producer_data(&self) -> ProducerData;
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ProducerData {
    pub name: String,
    pub producer_ref: AnyProducerRef,
}

#[derive(Resource, Default)]
pub struct ReferenceStore {
    assets: AssetOrFolder,
    game_sources: AssetOrFolder,
    variables: AssetOrFolder,
}

impl ReferenceStore {
    fn reload(&mut self, vars: &VariableFolder, assets: &AssetFolder) {
        self.variables = AssetOrFolder::from_vars(vars);
        self.game_sources = AssetOrFolder::from_game();
        self.assets = AssetOrFolder::from_asset_defs(assets);
    }

    fn get(&self, id: &ProducerId) -> Option<&ProducerData> {
        self.variables
            .get(id)
            .or_else(|| self.game_sources.get(id))
            .or_else(|| self.assets.get(id))
    }

    fn show_menu(
        &self,
        ui: &mut Ui,
        selected_asset: &mut Option<ProducerData>,
        is_type_allowed: &impl Fn(&ProducerData) -> bool,
    ) {
        self.assets.show_menu(ui, selected_asset, is_type_allowed);
        self.game_sources
            .show_menu(ui, selected_asset, is_type_allowed);
        self.variables
            .show_menu(ui, selected_asset, is_type_allowed);
    }
}

enum AssetOrFolder {
    Asset(ProducerData),
    Folder {
        name: String,
        assets: Vec<AssetOrFolder>,
    },
}
impl Default for AssetOrFolder {
    fn default() -> Self {
        Self::Folder {
            name: String::from(""),
            assets: Vec::new(),
        }
    }
}
impl AssetOrFolder {
    fn from_vars(vars: &VariableFolder) -> Self {
        Self::Folder {
            name: vars.name.clone(),
            assets: vars
                .content
                .iter()
                .map(|c| match c {
                    backend::style::variables::VariableOrFolder::Variable(t) => {
                        Self::Asset(t.producer_data().clone())
                    }
                    backend::style::variables::VariableOrFolder::Folder(f) => Self::from_vars(f),
                })
                .collect(),
        }
    }
    fn from_game() -> Self {
        Self::Folder {
            name: "Game".to_string(),
            assets: game_sources::get_game_sources()
                .into_iter()
                .map(|s| {
                    Self::Asset(ProducerData {
                        name: s.name.clone(),
                        producer_ref: s.producer_ref(),
                    })
                })
                .collect(),
        }
    }
    fn from_asset_defs(assets: &AssetFolder) -> Self {
        Self::Folder {
            name: assets.name.clone(),
            assets: assets
                .content
                .iter()
                .map(|a| match a {
                    backend::style::assets::AssetOrFolder::Asset(a) => {
                        AssetOrFolder::Asset(a.producer_data())
                    }
                    backend::style::assets::AssetOrFolder::Folder(f) => Self::from_asset_defs(f),
                })
                .collect(),
        }
    }
    fn get(&self, id: &ProducerId) -> Option<&ProducerData> {
        match self {
            AssetOrFolder::Asset(asset) => (&asset.producer_ref.id() == id).then_some(asset),
            AssetOrFolder::Folder { name: _, assets } => assets.iter().find_map(|a| a.get(id)),
        }
    }

    fn get_all_contained_ids(&self) -> Vec<&ProducerData> {
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
        selected_asset: &mut Option<ProducerData>,
        is_type_allowed: &impl Fn(&ProducerData) -> bool,
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
                        asset_id.producer_ref.ty().name()
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

fn savefile_changed(
    savefile: Res<Savefile>,
    mut reference_store: ResMut<ReferenceStore>,
    mut savefile_changed_event: EventReader<SavefileChanged>,
) {
    if savefile_changed_event.is_empty() {
        return;
    }
    savefile_changed_event.clear();

    info!("Reload reference store");
    reference_store.reload(&savefile.style().vars, &savefile.style().assets);
}

mod style {
    use backend::style::{assets::AssetDefinition, variables::VariableDefinition};

    use super::{IntoProducerData, ProducerData};

    impl IntoProducerData for VariableDefinition {
        fn producer_data(&self) -> ProducerData {
            ProducerData {
                name: self.name.clone(),
                producer_ref: self.producer_ref(),
            }
        }
    }

    impl IntoProducerData for AssetDefinition {
        fn producer_data(&self) -> ProducerData {
            ProducerData {
                name: self.name.clone(),
                producer_ref: self.producer_ref(),
            }
        }
    }
}

pub fn select_producer_reference(
    ui: &mut Ui,
    reference_store: &ReferenceStore,
    text: &str,
    is_type_allowed: impl Fn(&ProducerData) -> bool,
) -> InnerResponse<Option<AnyProducerRef>> {
    let mut selected_asset = None;
    let res = ui.menu_button(text, |ui| {
        reference_store.show_menu(ui, &mut selected_asset, &is_type_allowed);
    });
    InnerResponse::new(selected_asset.map(|prod| prod.producer_ref), res.response)
}

pub fn producer_id_editor(
    ui: &mut Ui,
    reference_store: &ReferenceStore,
    producer_id: &mut ProducerId,
    is_type_allowed: impl Fn(&ProducerData) -> bool,
) -> Response {
    let button_name = reference_store
        .get(producer_id)
        .map(|id| id.name.as_str())
        .unwrap_or("- Invalud Ref -");

    let mut res = select_producer_reference(ui, reference_store, button_name, is_type_allowed);
    if let Some(selected_producer) = res.inner {
        *producer_id = selected_producer.id();
        res.response.mark_changed();
    }

    res.response
}

pub fn any_producer_ref_editor(
    ui: &mut Ui,
    reference_store: &ReferenceStore,
    producer_ref: &mut AnyProducerRef,
    is_type_allowed: impl Fn(&ProducerData) -> bool,
) -> Response {
    let button_name = reference_store
        .get(&producer_ref.id())
        .map(|id| id.name.as_str())
        .unwrap_or("- Invalud Ref -");

    let mut res = select_producer_reference(ui, reference_store, button_name, is_type_allowed);
    if let Some(selected_producer) = res.inner {
        *producer_ref = selected_producer;
        res.response.mark_changed();
    }

    res.response
}
