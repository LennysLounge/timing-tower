use std::{fs, path::Path};

use bevy::{
    asset::{
        io::{file::FileAssetReader, AssetSourceBuilder},
        AssetApp,
    },
    ecs::{event::Event, system::Resource},
    prelude::Plugin,
};

use crate::style::StyleDefinition;

pub struct SaveFilePlugin;
impl Plugin for SaveFilePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_asset_source(
            "savefile",
            AssetSourceBuilder::default().with_reader(|| Box::new(FileAssetReader::new(""))),
        )
        .add_event::<SavefileLoaded>();
    }
}

#[derive(Event)]
pub struct SavefileLoaded;

#[derive(Resource)]
pub struct Savefile {
    pub style: StyleDefinition,
}

impl Savefile {
    pub fn load<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        let root_path = FileAssetReader::get_base_path().join(&path);

        let s = match fs::read_to_string(&root_path) {
            Err(e) => {
                eprintln!("Cannot read 'style.json': {}", e);
                panic!();
            }
            Ok(s) => s,
        };
        let mut style = match serde_json::from_str::<StyleDefinition>(&s) {
            Ok(o) => o,
            Err(e) => {
                println!("Error parsing json: {}", e);
                panic!();
            }
        };

        let Some(base_path) = path.as_ref().parent() else {
            panic!("Path has no parent");
        };

        // Update the paths of all assets.
        style.assets.all_t_mut().into_iter().for_each(|asset| {
            let asset_path = base_path.to_owned().join(asset.path.clone());
            asset.path = format!(
                "savefile://{}",
                asset_path
                    .into_os_string()
                    .into_string()
                    .expect("Path should be convertable into a string")
            );
        });
        Savefile { style }
    }
}
