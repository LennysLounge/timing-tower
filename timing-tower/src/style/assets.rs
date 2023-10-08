use bevy::{
    asset::LoadState,
    prelude::{AssetServer, Handle, Image},
};
use bevy_egui::egui::Ui;
use serde::{Deserialize, Serialize};
use tree_view::{TreeUi, TreeViewBuilder};
use unified_sim_model::model::Entry;
use uuid::Uuid;

use crate::{
    asset_reference_repo::AssetReferenceRepo,
    asset_repo::{AssetId, AssetRepo, AssetSource, AssetType, ImageSource, IntoAssetSource},
};

use super::{
    folder::{Folder, FolderActions},
    StyleTreeNode, StyleTreeUi, TreeViewAction,
};

#[derive(Serialize, Deserialize, Clone)]
pub enum AssetDefinition {
    Image(ImageAsset),
}

impl FolderActions for AssetDefinition {
    type FolderType = AssetDefinition;

    fn context_menu(
        ui: &mut Ui,
        folder: &Folder<Self::FolderType>,
        actions: &mut Vec<TreeViewAction>,
    ) {
        if ui.button("add image").clicked() {
            let image = AssetDefinition::Image(ImageAsset::new());
            actions.push(TreeViewAction::Select { node: *image.id() });
            actions.push(TreeViewAction::Insert {
                target: *folder.id(),
                node: Box::new(image),
                position: tree_view::DropPosition::Last,
            });
            ui.close_menu();
        }
    }
}

impl IntoAssetSource for AssetDefinition {
    fn get_asset_source(&self) -> AssetSource {
        match self {
            AssetDefinition::Image(i) => i.get_asset_source(),
        }
    }

    fn asset_id(&self) -> &AssetId {
        match self {
            AssetDefinition::Image(i) => i.asset_id(),
        }
    }
}

impl StyleTreeUi for AssetDefinition {
    fn tree_view(&mut self, ui: &mut TreeUi, actions: &mut Vec<TreeViewAction>) {
        match self {
            AssetDefinition::Image(o) => o.tree_view(ui, actions),
        }
    }

    fn property_editor(&mut self, ui: &mut Ui, asset_repo: &AssetReferenceRepo) {
        match self {
            AssetDefinition::Image(o) => o.property_editor(ui, asset_repo),
        }
    }
}

impl StyleTreeNode for AssetDefinition {
    fn id(&self) -> &uuid::Uuid {
        match self {
            AssetDefinition::Image(o) => o.id(),
        }
    }

    fn chidren(&self) -> Vec<&dyn StyleTreeNode> {
        match self {
            AssetDefinition::Image(o) => o.chidren(),
        }
    }

    fn chidren_mut(&mut self) -> Vec<&mut dyn StyleTreeNode> {
        match self {
            AssetDefinition::Image(o) => o.chidren_mut(),
        }
    }

    fn can_insert(&self, node: &dyn std::any::Any) -> bool {
        match self {
            AssetDefinition::Image(o) => o.can_insert(node),
        }
    }

    fn remove(&mut self, id: &uuid::Uuid) -> Option<Box<dyn std::any::Any>> {
        match self {
            AssetDefinition::Image(o) => o.remove(id),
        }
    }

    fn insert(&mut self, node: Box<dyn std::any::Any>, position: &tree_view::DropPosition) {
        match self {
            AssetDefinition::Image(o) => o.insert(node, position),
        }
    }
}

impl AssetDefinition {
    pub fn load_asset(&mut self, asset_server: &AssetServer) {
        match self {
            AssetDefinition::Image(o) => o.load_asset(asset_server),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ImageAsset {
    pub id: AssetId,
    pub path: String,
    #[serde(skip)]
    handle: Option<Handle<Image>>,
    #[serde(skip)]
    #[serde(default = "default_load_state")]
    load_state: LoadState,
}

impl IntoAssetSource for ImageAsset {
    fn get_asset_source(&self) -> AssetSource {
        AssetSource::Image(Box::new(StaticImage(self.handle.clone())))
    }

    fn asset_id(&self) -> &AssetId {
        &self.id
    }
}

impl StyleTreeUi for ImageAsset {
    fn tree_view(&mut self, ui: &mut TreeUi, _actions: &mut Vec<TreeViewAction>) {
        TreeViewBuilder::leaf(self.id.id).show(ui, |ui| {
            ui.label(&self.id.name);
        });
    }

    fn property_editor(&mut self, ui: &mut Ui, _asset_repo: &AssetReferenceRepo) {
        ui.label("Name");
        let res = ui.text_edit_singleline(&mut self.id.name);
        if res.changed() {
            println!(" was changed");
        }
        ui.separator();
        ui.label("Path:");
        ui.text_edit_singleline(&mut self.path);
        match self.load_state {
            LoadState::NotLoaded => ui.label("Asset is not loaded"),
            LoadState::Loading => ui.label("Asset is loading"),
            LoadState::Loaded => ui.label("Asset loaded correctly"),
            LoadState::Failed => ui.label(
                "Failed to load the asset. Make sure the path is pointing to a valid image file.",
            ),
            LoadState::Unloaded => ui.label("The asset was unloaded"),
        };
    }
}

impl StyleTreeNode for ImageAsset {
    fn id(&self) -> &Uuid {
        &self.id.id
    }

    fn chidren(&self) -> Vec<&dyn StyleTreeNode> {
        Vec::new()
    }

    fn chidren_mut(&mut self) -> Vec<&mut dyn StyleTreeNode> {
        Vec::new()
    }

    fn can_insert(&self, _node: &dyn std::any::Any) -> bool {
        false
    }

    fn remove(&mut self, _id: &Uuid) -> Option<Box<dyn std::any::Any>> {
        None
    }

    fn insert(&mut self, _node: Box<dyn std::any::Any>, _position: &tree_view::DropPosition) {}
}

impl ImageAsset {
    fn new() -> Self {
        Self {
            id: AssetId {
                id: Uuid::new_v4(),
                name: "new image".to_string(),
                asset_type: AssetType::Image,
            },
            path: String::new(),
            handle: None,
            load_state: default_load_state(),
        }
    }

    pub fn load_asset(&mut self, asset_server: &AssetServer) {
        self.handle = Some(asset_server.load(&self.path));
        if let Some(handle) = self.handle.as_ref() {
            self.load_state = asset_server.get_load_state(handle);
        }
    }
}

fn default_load_state() -> LoadState {
    LoadState::NotLoaded
}

pub struct StaticImage(pub Option<Handle<Image>>);
impl ImageSource for StaticImage {
    fn resolve(&self, _vars: &AssetRepo, _entry: Option<&Entry>) -> Option<Handle<Image>> {
        self.0.clone()
    }
}
