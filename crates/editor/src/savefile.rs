use bevy::{
    asset::io::{file::FileAssetReader, AssetReader, AssetSourceBuilder},
    prelude::{AssetApp, Plugin, Startup},
};

pub struct SaveFilePlugin;
impl Plugin for SaveFilePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_asset_source(
            "savefile",
            AssetSourceBuilder {
                reader: Some(Box::new(|| {
                    Box::new(SaveFileReader {
                        file_reader: FileAssetReader::new("savefile"),
                    })
                })),
                writer: None,
                watcher: None,
                processed_reader: None,
                processed_writer: None,
                processed_watcher: None,
            },
        )
        .add_systems(Startup, setup);
    }
}

fn setup() {}

struct SaveFileReader {
    file_reader: FileAssetReader,
}
impl AssetReader for SaveFileReader {
    fn read<'a>(
        &'a self,
        path: &'a std::path::Path,
    ) -> bevy::utils::BoxedFuture<
        'a,
        Result<Box<bevy::asset::io::Reader<'a>>, bevy::asset::io::AssetReaderError>,
    > {
        self.file_reader.read(path)
    }

    fn read_meta<'a>(
        &'a self,
        path: &'a std::path::Path,
    ) -> bevy::utils::BoxedFuture<
        'a,
        Result<Box<bevy::asset::io::Reader<'a>>, bevy::asset::io::AssetReaderError>,
    > {
        self.file_reader.read_meta(path)
    }

    fn read_directory<'a>(
        &'a self,
        path: &'a std::path::Path,
    ) -> bevy::utils::BoxedFuture<
        'a,
        Result<Box<bevy::asset::io::PathStream>, bevy::asset::io::AssetReaderError>,
    > {
        self.file_reader.read_directory(path)
    }

    fn is_directory<'a>(
        &'a self,
        path: &'a std::path::Path,
    ) -> bevy::utils::BoxedFuture<'a, Result<bool, bevy::asset::io::AssetReaderError>> {
        self.file_reader.is_directory(path)
    }
}
