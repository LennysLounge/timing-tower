use std::collections::HashMap;

use bevy::{
    app::{Plugin, Update},
    ecs::{
        component::Component,
        entity::Entity,
        schedule::{IntoSystemConfigs, SystemSet},
        system::{Commands, Local, Query, Res, ResMut, Resource},
    },
    hierarchy::DespawnRecursiveExt,
    math::vec3,
    utils::hashbrown::HashSet,
};

use unified_sim_model::model::{Entry, Model};

use crate::{
    savefile::Savefile,
    style::{
        graphic::{
            graphic_items::{entry_context::EntrySelection, ComputedGraphicItem},
            GraphicStateId,
        },
        StyleId, StyleItem,
    },
    style_batcher::{CellId, StyleBatcher},
    tree_iterator::TreeIterator,
    value_store::ValueStore,
    GameAdapterResource,
};

use self::{
    graphic_item_data_storage::{GraphicItemDataStorage, GraphicItemDataStorageContext},
    style_resolver::StyleResolver,
};

mod compute_style;
mod graphic_item_data_storage;
mod style_resolver;

pub struct GraphicPlugin;
impl Plugin for GraphicPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<GraphicStates>().add_systems(
            Update,
            (spawn_or_delete_graphics, update_graphics)
                .chain()
                .in_set(StyleElementUpdate),
        );
    }
}

#[derive(Resource, Default)]
pub struct GraphicStates {
    pub states: HashMap<StyleId, GraphicStateId>,
}

#[derive(SystemSet, Hash, Debug, PartialEq, Eq, Clone)]
pub struct StyleElementUpdate;

#[derive(Component)]
pub struct Graphic {
    id: StyleId,
}

fn spawn_or_delete_graphics(
    mut commands: Commands,
    savefile: Res<Savefile>,
    mut id_to_entity_map: Local<HashMap<StyleId, Entity>>,
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
    graphic_states: Res<GraphicStates>,
    savefile: Res<Savefile>,
    mut batcher: ResMut<StyleBatcher>,
    mut graphic_item_data_storage: Local<GraphicItemDataStorage>,
    value_store: Res<ValueStore>,
    game_adapter: Res<GameAdapterResource>,
) {
    let Some(game_adapter) = &game_adapter.adapter else {
        graphic_item_data_storage.clear();
        return;
    };

    let model = game_adapter.model.read_raw();
    let Some(session) = model.current_session() else {
        return;
    };

    for graphic in graphics.iter_mut() {
        savefile.style().search(graphic.id, |item| {
            if let StyleItem::Graphic(graphic) = item {
                let computed_style = graphic.compute_style(graphic_states.states.get(&graphic.id));
                let resolver = StyleResolver::new(&*value_store, session);
                update_graphic_item(
                    &computed_style.root,
                    &mut *batcher,
                    &mut graphic_item_data_storage.make_context(0),
                    &resolver,
                    &*model,
                );
            }
        });
    }

    // Remove stale data
    graphic_item_data_storage.clear_stale_data();
}

fn update_graphic_item(
    item: &ComputedGraphicItem,
    batcher: &mut StyleBatcher,
    graphic_item_data_storage: &mut GraphicItemDataStorageContext<'_>,
    resolver: &StyleResolver,
    _model: &Model,
) {
    match item {
        ComputedGraphicItem::Root(root) => {
            let new_resolver = resolver.clone().with_position(vec3(
                resolver.property(&root.position.x).unwrap_or_default().0,
                -resolver.property(&root.position.y).unwrap_or_default().0,
                0.0,
            ));
            root.items.iter().for_each(|item| {
                update_graphic_item(
                    item,
                    batcher,
                    graphic_item_data_storage,
                    &new_resolver,
                    _model,
                )
            });
        }
        ComputedGraphicItem::Cell(cell) => {
            let cell_id = graphic_item_data_storage.get_or_create(cell.id, || CellId::new());
            batcher.add(&cell_id, resolver.cell(&cell));
        }
        ComputedGraphicItem::ClipArea(clip_area) => {
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
        ComputedGraphicItem::DriverTable(driver_table) => {
            let DriverTableData { scroll_position } =
                graphic_item_data_storage.get_or_default(driver_table.id);

            let position = vec3(
                resolver
                    .property(&driver_table.position.x)
                    .unwrap_or_default()
                    .0,
                -resolver
                    .property(&driver_table.position.y)
                    .unwrap_or_default()
                    .0,
                0.0,
            );

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
            entries.sort_by_key(|e| *e.position);

            // Update scroll position to make sure the focused entry is visible
            if let Some(focused_entry_index) = entries.iter().position(|entry| entry.focused) {
                let rows_to_skip = (focused_entry_index as f32 - 12.0)
                    .min(entries.len() as f32 - 23.0)
                    .max(0.0);
                *scroll_position = *scroll_position - (*scroll_position - rows_to_skip) * 0.2;
            }
            let scroll_offset = row_offset * *scroll_position - position;

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
        ComputedGraphicItem::EntryContext(entry_context) => {
            let mut entries: Vec<&Entry> = resolver.session().entries.values().collect();
            entries.sort_by_key(|e| *e.position);
            let focused_index = entries.iter().position(|e| e.focused);

            let entry = match entry_context.selection {
                EntrySelection::First => entries.get(0),
                EntrySelection::Second => entries.get(1),
                EntrySelection::Third => entries.get(2),
                EntrySelection::AheadOfFocus => focused_index.and_then(|idx| entries.get(idx - 1)),
                EntrySelection::Focus => focused_index.and_then(|idx| entries.get(idx)),
                EntrySelection::BehindFocus => focused_index.and_then(|idx| entries.get(idx + 1)),
            };

            let mut new_resolver = resolver.clone();
            if let Some(entry) = entry {
                new_resolver = new_resolver.with_entry(entry);
            }

            for item in entry_context.items.iter() {
                update_graphic_item(
                    item,
                    batcher,
                    graphic_item_data_storage,
                    &new_resolver,
                    _model,
                );
            }
        }
    }
}

#[derive(Default)]
struct DriverTableData {
    scroll_position: f32,
}
