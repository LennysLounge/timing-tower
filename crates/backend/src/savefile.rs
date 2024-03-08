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

use crate::{
    exact_variant::ExactVariant,
    style::{StyleDefinition, StyleItem},
};

pub struct SavefilePlugin;
impl Plugin for SavefilePlugin {
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
pub struct SavefileChanged {
    pub replace: bool,
}

#[derive(Resource)]
pub struct Savefile {
    style: ExactVariant<StyleItem, StyleDefinition>,
    base_path: PathBuf,
    working_directory_path: PathBuf,
}
impl Default for Savefile {
    fn default() -> Self {
        Self {
            style: StyleDefinition::default().into(),
            base_path: Default::default(),
            working_directory_path: Default::default(),
        }
    }
}

impl Savefile {
    fn new<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        let style_file = FileAssetReader::get_base_path().join(&path);

        let s = match fs::read_to_string(&style_file) {
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
        let Some(working_directory_path) = style_file.parent() else {
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
            style: style.into(),
            base_path: base_path.to_owned(),
            working_directory_path: working_directory_path.to_owned(),
        }
    }

    pub fn load<P>(&mut self, path: P, mut event: EventWriter<SavefileChanged>)
    where
        P: AsRef<Path>,
    {
        *self = Self::new(path);
        event.send(SavefileChanged { replace: true });
    }

    pub fn set(
        &mut self,
        new_style: ExactVariant<StyleItem, StyleDefinition>,
        event: &mut EventWriter<SavefileChanged>,
    ) {
        self.style = new_style.into();
        event.send(SavefileChanged { replace: false });
    }

    pub fn style(&self) -> &ExactVariant<StyleItem, StyleDefinition> {
        &self.style
    }

    pub fn style_mut(&mut self) -> &mut ExactVariant<StyleItem, StyleDefinition> {
        &mut self.style
    }

    pub fn base_path(&self) -> &Path {
        self.base_path.as_path()
    }

    pub fn working_directory_path(&self) -> &Path {
        self.working_directory_path.as_path()
    }
}
