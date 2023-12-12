use std::{
    fs,
    path::{Path, PathBuf},
};

use bevy::{
    asset::{
        io::{file::FileAssetReader, AssetSourceBuilder},
        AssetApp,
    },
    ecs::{
        event::{Event, EventWriter},
        system::Resource,
    },
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
        .init_resource::<Savefile>()
        .add_event::<SavefileChanged>();
    }
}

#[derive(Event)]
pub struct SavefileChanged;

#[derive(Resource, Default)]
pub struct Savefile {
    style: StyleDefinition,
    base_path: PathBuf,
}

impl Savefile {
    fn new<P>(path: P) -> Self
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
        let style = match serde_json::from_str::<StyleDefinition>(&s) {
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
        // style.assets.all_t_mut().into_iter().for_each(|asset| {
        //     let asset_path = base_path.to_owned().join(asset.path.clone());
        //     asset.path = format!(
        //         "savefile://{}",
        //         asset_path
        //             .into_os_string()
        //             .into_string()
        //             .expect("Path should be convertable into a string")
        //     );
        // });
        Savefile {
            style,
            base_path: base_path.to_owned(),
        }
    }

    pub fn load<P>(&mut self, path: P, mut event: EventWriter<SavefileChanged>)
    where
        P: AsRef<Path>,
    {
        *self = Self::new(path);
        event.send(SavefileChanged);
    }

    pub fn set(&mut self, new_style: StyleDefinition, event: &mut EventWriter<SavefileChanged>) {
        self.style = new_style;
        event.send(SavefileChanged);
    }

    pub fn style(&self) -> &StyleDefinition {
        &self.style
    }

    pub fn base_path(&self) -> &Path {
        self.base_path.as_path()
    }
}
