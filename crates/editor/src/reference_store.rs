use bevy::{
    app::{First, Plugin},
    ecs::{
        event::EventReader,
        system::{Res, ResMut, Resource},
    },
};
use bevy_egui::egui::{
    vec2, Align2, CollapsingHeader, Color32, InnerResponse, Response, RichText, ScrollArea, Ui,
};
use serde::{Deserialize, Serialize};
use tracing::info;

use backend::{
    game_sources,
    savefile::{Savefile, SavefileChanged},
    style::{assets::AssetFolder, variables::VariableFolder, StyleDefinition},
    value_store::ProducerId,
    value_types::AnyProducerRef,
};

use crate::ui::popup::Popup;

pub struct ReferenceStorePlugin;
impl Plugin for ReferenceStorePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(ReferenceStore::default())
            .add_systems(First, savefile_changed);
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
    *reference_store = ReferenceStore {
        entries: Entry::from_style_definition(savefile.style()),
    };
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ProducerData {
    pub name: String,
    pub producer_ref: AnyProducerRef,
}

#[derive(Resource, Default)]
pub struct ReferenceStore {
    entries: Vec<Entry>,
}

impl ReferenceStore {
    fn get(&self, id: &ProducerId) -> Option<&ProducerData> {
        self.entries.iter().find_map(|e| e.get(id))
    }

    pub fn show_popup(
        &self,
        ui: &mut Ui,
        text: &str,
        is_type_allowed: impl Fn(&ProducerData) -> bool,
    ) -> InnerResponse<Option<AnyProducerRef>> {
        let mut selected_producer: Option<ProducerData> = None;

        let button = ui.button(text);
        Popup::new(
            ui.next_auto_id().with("ReferenceStore_popup"),
            button.rect.left_bottom(),
        )
        .should_toggle(button.clicked())
        .pivot(Align2::CENTER_TOP)
        .show(ui, |ui, _| {
            ui.set_min_size(vec2(300.0, 200.0));
            ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    self.entries
                        .iter()
                        .for_each(|e| e.show_menu(ui, &mut selected_producer, &is_type_allowed));
                });
        });
        InnerResponse::new(selected_producer.map(|prod| prod.producer_ref), button)
    }
}

enum Entry {
    Producer(ProducerData),
    Folder { name: String, entries: Vec<Entry> },
}
impl Default for Entry {
    fn default() -> Self {
        Self::Folder {
            name: String::from(""),
            entries: Vec::new(),
        }
    }
}
impl Entry {
    fn from_style_definition(style: &StyleDefinition) -> Vec<Self> {
        let mut result = Vec::new();
        result.push(Self::from_assets(&style.assets));
        result.push(Self::from_game());
        result.push(Self::from_vars(&style.vars));
        result
    }
    fn from_vars(vars: &VariableFolder) -> Self {
        Self::Folder {
            name: vars.name.clone(),
            entries: vars
                .content
                .iter()
                .map(|c| match c {
                    backend::style::variables::VariableOrFolder::Variable(var) => {
                        Self::Producer(ProducerData {
                            name: var.name.clone(),
                            producer_ref: var.producer_ref(),
                        })
                    }
                    backend::style::variables::VariableOrFolder::Folder(f) => Self::from_vars(f),
                })
                .collect(),
        }
    }
    fn from_game() -> Self {
        Self::Folder {
            name: "Game".to_string(),
            entries: game_sources::get_game_sources()
                .into_iter()
                .map(|s| {
                    Self::Producer(ProducerData {
                        name: s.name.clone(),
                        producer_ref: s.producer_ref(),
                    })
                })
                .collect(),
        }
    }
    fn from_assets(assets: &AssetFolder) -> Self {
        Self::Folder {
            name: assets.name.clone(),
            entries: assets
                .content
                .iter()
                .map(|a| match a {
                    backend::style::assets::AssetOrFolder::Asset(a) => {
                        Entry::Producer(ProducerData {
                            name: a.name.clone(),
                            producer_ref: a.producer_ref(),
                        })
                    }
                    backend::style::assets::AssetOrFolder::Folder(f) => Self::from_assets(f),
                })
                .collect(),
        }
    }
    fn get(&self, id: &ProducerId) -> Option<&ProducerData> {
        match self {
            Entry::Producer(asset) => (&asset.producer_ref.id() == id).then_some(asset),
            Entry::Folder {
                name: _,
                entries: assets,
            } => assets.iter().find_map(|a| a.get(id)),
        }
    }

    fn get_all_contained_ids(&self) -> Vec<&ProducerData> {
        match self {
            Entry::Producer(id) => vec![id],
            Entry::Folder {
                name: _,
                entries: assets,
            } => assets
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
            Entry::Producer(asset_id) => {
                let is_asset_allowed = is_type_allowed(&asset_id);
                let res = ui.add_enabled_ui(is_asset_allowed, |ui| {
                    ui.horizontal(|ui| {
                        let res = ui.label(
                            RichText::new(asset_id.producer_ref.ty().name())
                                .color(Color32::from_gray(120)),
                        );
                        ui.add_space(60.0 - res.rect.width());
                        if ui.selectable_label(false, &asset_id.name).clicked() {
                            *selected_asset = Some(asset_id.clone());
                            ui.close_menu();
                        }
                    });
                });
                if !is_asset_allowed {
                    res.response.on_hover_text(format!(
                        "{} type not allowed for this reference.",
                        asset_id.producer_ref.ty().name()
                    ));
                }
            }
            Entry::Folder {
                name,
                entries: assets,
            } => {
                let is_any_allowed = self
                    .get_all_contained_ids()
                    .into_iter()
                    .any(|a| is_type_allowed(a));

                let res = ui.add_enabled_ui(is_any_allowed, |ui| {
                    CollapsingHeader::new(name).show(ui, |ui| {
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

    let mut res = reference_store.show_popup(ui, button_name, is_type_allowed);
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

    let mut res = reference_store.show_popup(ui, button_name, is_type_allowed);
    if let Some(selected_producer) = res.inner {
        *producer_ref = selected_producer;
        res.response.mark_changed();
    }

    res.response
}
