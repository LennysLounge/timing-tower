use bevy::{
    app::Update,
    asset::{Asset, AssetApp, AssetEvent, AssetId, AssetLoader, AsyncReadExt},
    ecs::event::{Event, EventReader, EventWriter},
    prelude::Plugin,
    reflect::TypePath,
};

use crate::style::StyleDefinition;

pub struct SaveFilePlugin;
impl Plugin for SaveFilePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_asset::<Savefile>()
            .add_event::<SavefileLoaded>()
            .init_asset_loader::<JsonSavefileLoader>()
            .add_systems(Update, listen_for_savefile_assets_events);
    }
}

#[derive(Event)]
pub struct SavefileLoaded {
    savefile: AssetId<Savefile>,
}

#[derive(Asset, TypePath)]
pub struct Savefile {
    _style: StyleDefinition,
}

#[derive(Default)]
struct JsonSavefileLoader;
impl AssetLoader for JsonSavefileLoader {
    type Asset = Savefile;
    type Settings = ();
    type Error = std::io::Error;

    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        _settings: &'a Self::Settings,
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            println!("load savefile from path: {:?}", load_context.path());

            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;

            let mut style = serde_json::from_slice::<StyleDefinition>(&bytes)?;

            let base_path = load_context
                .path()
                .parent()
                .map(|p| p.to_owned())
                .expect("Path has no parent");

            // Update the paths of all assets.
            style.assets.all_t_mut().into_iter().for_each(|asset| {
                let mut asset_path = base_path.clone();
                asset_path.push(asset.path.clone());
                asset.path = asset_path
                    .into_os_string()
                    .into_string()
                    .expect("Path should be convertable into a string");
            });

            Ok(Savefile { _style: style })
        })
    }

    fn extensions(&self) -> &[&str] {
        &["style.json"]
    }
}

pub fn listen_for_savefile_assets_events(
    mut events: EventReader<AssetEvent<Savefile>>,
    mut send_event: EventWriter<SavefileLoaded>,
) {
    for event in events.read() {
        if let AssetEvent::Added { id } = event {
            send_event.send(SavefileLoaded {
                savefile: id.clone(),
            });
        }
    }
}
