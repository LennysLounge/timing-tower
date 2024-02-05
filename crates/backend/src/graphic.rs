use std::collections::HashMap;

use bevy::{
    app::{Plugin, Update},
    ecs::{
        component::Component,
        entity::Entity,
        schedule::{IntoSystemConfigs, SystemSet},
        system::{Commands, Local, Query, Res, ResMut},
    },
    hierarchy::DespawnRecursiveExt,
    math::vec3,
    utils::hashbrown::HashSet,
};

use unified_sim_model::model::{Entry, Model};
use uuid::Uuid;

use crate::{
    savefile::Savefile,
    style::{graphic_items::GraphicItem, StyleItem, StyleItemRef},
    style_batcher::{CellId, StyleBatcher},
    tree_iterator::TreeIterator,
    value_store::ValueStore,
    GameAdapterResource,
};

use self::{
    graphic_item_data_storage::{GraphicItemDataStorage, GraphicItemDataStorageContext},
    style_resolver::StyleResolver,
};

mod graphic_item_data_storage;
mod style_resolver;

pub struct GraphicPlugin;
impl Plugin for GraphicPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (spawn_or_delete_graphics, update_graphics)
                .chain()
                .in_set(StyleElementUpdate),
        );
    }
}

#[derive(SystemSet, Hash, Debug, PartialEq, Eq, Clone)]
pub struct StyleElementUpdate;

#[derive(Component)]
pub struct Graphic {
    id: Uuid,
}

fn spawn_or_delete_graphics(
    mut commands: Commands,
    savefile: Res<Savefile>,
    mut id_to_entity_map: Local<HashMap<Uuid, Entity>>,
) {
    let mut known_graphics = HashSet::new();
    // spawn new graphics
    for graphic in savefile.style().graphics.contained_graphics() {
        if !id_to_entity_map.contains_key(&graphic.id) {
            let entity = commands.spawn(Graphic { id: graphic.id }).id();
            id_to_entity_map.insert(graphic.id, entity);
        }
        known_graphics.insert(graphic.id);
    }
    // delete graphic that are no longer in the style
    id_to_entity_map.retain(|id, entity| {
        if !known_graphics.contains(id) {
            commands.entity(*entity).despawn_recursive();
            false
        } else {
            true
        }
    });
}

fn update_graphics(
    mut graphics: Query<&mut Graphic>,
    savefile: Res<Savefile>,
    mut batcher: ResMut<StyleBatcher>,
    mut graphic_item_data_storage: Local<GraphicItemDataStorage>,
    value_store: Res<ValueStore>,
    game_adapter: Res<GameAdapterResource>,
) {
    let model = game_adapter
        .adapter
        .model
        .read()
        .expect("Unified model cannot be locked for read");

    let Some(session) = model.current_session() else {
        return;
    };

    for graphic in graphics.iter_mut() {
        savefile.style().as_ref().search(graphic.id, |item| {
            if let StyleItemRef::Graphic(graphic) = item {
                let mut resolver = StyleResolver::new(&*value_store, session);
                resolver.set_position(vec3(
                    resolver
                        .property(&graphic.items.position.x)
                        .unwrap_or_default()
                        .0,
                    -resolver
                        .property(&graphic.items.position.y)
                        .unwrap_or_default()
                        .0,
                    0.0,
                ));
                graphic.items.items.iter().for_each(|item| {
                    update_graphic_item(
                        item,
                        &mut *batcher,
                        &mut graphic_item_data_storage.make_context(0),
                        &resolver,
                        &*model,
                    );
                });
            }
        });
    }

    // Remove stale data
    graphic_item_data_storage.clear_stale_data();
}

fn update_graphic_item(
    item: &GraphicItem,
    batcher: &mut StyleBatcher,
    graphic_item_data_storage: &mut GraphicItemDataStorageContext<'_>,
    resolver: &StyleResolver,
    _model: &Model,
) {
    match item {
        GraphicItem::Cell(cell) => {
            let cell_id = graphic_item_data_storage.get_or_create(cell.id, || CellId::new());
            batcher.add(&cell_id, resolver.cell(&cell));
        }
        GraphicItem::ClipArea(clip_area) => {
            let cell_id = graphic_item_data_storage.get_or_create(clip_area.id, || CellId::new());
            let clip_area_style = resolver.clip_area(&clip_area);
            let new_resolver = resolver
                .clone()
                .with_position(clip_area_style.pos)
                .with_render_layer(clip_area_style.render_layer);
            batcher.add_clip_area(&cell_id, clip_area_style);
            for item in clip_area.items.iter() {
                update_graphic_item(
                    item,
                    batcher,
                    graphic_item_data_storage,
                    &new_resolver,
                    _model,
                );
            }
        }
        GraphicItem::DriverTable(driver_table) => {
            let DriverTableData { scroll_position } =
                graphic_item_data_storage.get_or_default(driver_table.id);

            // Read the row offset.
            let row_offset = vec3(
                resolver
                    .property(&driver_table.row_offset.x)
                    .unwrap_or_default()
                    .0,
                -resolver
                    .property(&driver_table.row_offset.y)
                    .unwrap_or_default()
                    .0,
                0.0,
            );

            // Get entries sorted by position
            let mut entries: Vec<&Entry> = resolver.session().entries.values().collect();
            entries.sort_by(|e1, e2| {
                let is_connected = e2.connected.cmp(&e1.connected);
                let position = e1
                    .position
                    .partial_cmp(&e2.position)
                    .unwrap_or(std::cmp::Ordering::Equal);
                is_connected.then(position)
            });

            // Update scroll position to make sure the focused entry is visible
            if let Some(focused_entry_index) = entries.iter().position(|entry| entry.focused) {
                let rows_to_skip = (focused_entry_index as f32 - 12.0)
                    .min(entries.len() as f32 - 23.0)
                    .max(0.0);
                *scroll_position = *scroll_position - (*scroll_position - rows_to_skip) * 0.2;
            }
            let scroll_offset = row_offset * *scroll_position;

            // Each column for all entries.
            for (index, entry) in entries.iter().enumerate() {
                let new_resolver = resolver
                    .clone()
                    .with_position(*resolver.position() - scroll_offset + row_offset * index as f32)
                    .with_entry(entry);
                for column in driver_table.columns.iter() {
                    update_graphic_item(
                        column,
                        batcher,
                        &mut graphic_item_data_storage.make_context(entry.id),
                        &new_resolver,
                        _model,
                    );
                }
            }
        }
    }
}

#[derive(Default)]
struct DriverTableData {
    scroll_position: f32,
}
