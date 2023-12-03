use std::{
    collections::HashMap,
    env,
    fs::{self},
};

use asset_store_impl::AssetStoreImpl;
use backend::{style::StyleDefinition, value_store::ValueStore};
use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    ecs::system::EntityCommand,
    prelude::{
        App, AssetServer, ClearColor, Color, Commands, Component, EntityWorldMut, EventWriter,
        Handle, PreStartup, Res, Resource, Startup, Vec2, Vec3, World,
    },
    text::Font,
    time::{Timer, TimerMode},
    DefaultPlugins,
};
use bevy_egui::EguiPlugin;
use common::{
    asset_store::AssetStore,
    cell::{
        init_cell,
        style::{CellStyle, SetStyle, TextAlignment},
        CellPlugin,
    },
    gradient_material::CustomMaterialPlugin,
};
use editor::{EditorPlugin, EditorState};

use savefile::SaveFilePlugin;

use timing_tower::{init_timing_tower, TimingTowerPlugin};
use unified_sim_model::Adapter;
use uuid::uuid;

mod asset_store_impl;
mod editor;
mod properties;
mod reference_store;
mod savefile;
mod style;
mod timing_tower;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    App::new()
        .insert_resource(ClearColor(Color::rgba(0.1, 0.1, 0.1, 0.0)))
        .insert_resource(SimpleTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
        // Crate plugins
        // Plugins
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(EguiPlugin)
        // Crate plugins
        .add_plugins(SaveFilePlugin)
        .add_plugins(CellPlugin)
        .add_plugins(CustomMaterialPlugin)
        .add_plugins(TimingTowerPlugin)
        .add_plugins(EditorPlugin)
        // Systems
        .add_systems(PreStartup, load)
        .add_systems(Startup, setup)
        .run();
}

#[derive(Resource)]
struct SimpleTimer(Timer);

#[derive(Resource)]
pub struct DefaultFont(pub Handle<Font>);

#[derive(Resource)]
#[allow(unused)]
pub struct GameAdapterResource {
    adapter: Adapter,
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
struct BackgroundImage;

fn load(asset_server: Res<AssetServer>, mut commands: Commands) {
    //let font: Handle<Font> = asset_server.load("fonts/FiraSans-bold.ttf");
    //let font: Handle<Font> = asset_server.load("fonts/Heebo-Regular.ttf");
    //let font: Handle<Font> = asset_server.load("fonts/Heebo-Black.ttf");
    let font: Handle<Font> = asset_server.load("fonts/cufonfonts D-DIN-Bold.otf");
    commands.insert_resource(DefaultFont(font));
}

fn setup(
    mut commands: Commands,
    mut set_style_event: EventWriter<SetStyle>,
    asset_server: Res<AssetServer>,
) {
    let adapter = Adapter::new_dummy();
    commands.insert_resource(GameAdapterResource {
        adapter: adapter.clone(),
    });

    let s = match fs::read_to_string("crates/editor/savefile/style.style.json") {
        Err(e) => {
            eprintln!("Cannot read 'style.json': {}", e);
            return;
        }
        Ok(s) => s,
    };
    let style = match serde_json::from_str::<StyleDefinition>(&s) {
        Ok(o) => o,
        Err(e) => {
            println!("Error parsing json: {}", e);
            return;
        }
    };

    // style
    //     .assets
    //     .all_t_mut()
    //     .into_iter()
    //     .for_each(|a| a.load_asset(&*asset_server));

    let background_id = commands
        .spawn_empty()
        .add(init_cell)
        .insert(BackgroundImage)
        .id();
    set_style_event.send(SetStyle {
        entity: background_id,
        style: CellStyle {
            text: "".to_string(),
            text_color: Color::BLACK,
            text_size: 20.0,
            text_alignment: TextAlignment::Center,
            text_position: Vec2::ZERO,
            color: Color::WHITE,
            texture: Some(uuid!("819d2f30-0d03-413a-8f09-9a0afa58b3ed")),
            pos: Vec3::new(0.0, 0.0, 0.0),
            size: Vec2::new(1920.0, 1080.0),
            skew: 0.0,
            visible: true,
            rounding: [0.0, 0.0, 0.0, 0.0],
        },
    });

    commands.insert_resource(AssetStore::new(Box::new(AssetStoreImpl::new(
        &style.assets,
        &asset_server,
    ))));

    commands.insert_resource(style.clone());
    let mut repo = ValueStore {
        assets: HashMap::new(),
    };
    repo.reload_repo(style.vars.all_t(), style.assets.all_t());
    commands.insert_resource(repo);

    commands.insert_resource(EditorState::new());

    commands.spawn_empty().add(init_timing_tower(adapter));
}

pub trait SpawnAndInitWorld {
    fn spawn_new<C: EntityCommand>(&mut self, command: C) -> EntityWorldMut;
}

impl SpawnAndInitWorld for World {
    fn spawn_new<C: EntityCommand>(&mut self, command: C) -> EntityWorldMut {
        let id = self.spawn_empty().id();
        command.apply(id, self);
        self.entity_mut(id)
    }
}
