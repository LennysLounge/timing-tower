use std::collections::HashMap;

use bevy::{
    app::{Plugin, Update},
    ecs::{
        component::Component,
        entity::Entity,
        schedule::IntoSystemConfigs,
        system::{Commands, Local, Query, Res, ResMut},
    },
    hierarchy::DespawnRecursiveExt,
    math::{vec2, vec3, Vec2, Vec3},
    render::color::Color,
    utils::hashbrown::HashSet,
};
use common::communication::{CellStyle, ClipAreaStyle};
use unified_sim_model::model::{Entry, EntryId, Model, Session};
use uuid::Uuid;

use crate::{
    savefile::Savefile,
    style::{
        cell::{Cell, ClipArea},
        graphic_items::GraphicItem,
        StyleItem, StyleItemRef,
    },
    style_batcher::{CellId, StyleBatcher},
    timing_tower::StyleElementUpdate,
    tree_iterator::TreeIterator,
    value_store::{TypedValueResolver, ValueStore},
    value_types::{Boolean, Font, Number, Property, Text, Texture, Tint, Vec2Property},
    GameAdapterResource,
};

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

#[derive(Component)]
pub struct Graphic {
    id: Uuid,
}

enum GraphicItemData {
    Cell {
        cell_id: CellId,
        updated: bool,
    },
    ClipArea {
        cell_id: CellId,
        updated: bool,
        data: HashMap<Uuid, GraphicItemData>,
    },
    DriverTable {
        updated: bool,
        rows: HashMap<EntryId, HashMap<Uuid, GraphicItemData>>,
        scroll_position: f32,
    },
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
    mut graphic_item_data: Local<HashMap<Uuid, GraphicItemData>>,
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
                        &mut *graphic_item_data,
                        &resolver,
                        &*model,
                    );
                });
            }
        });
    }

    // Remove stale data
    // graphic_item_data.retain(|_, data| data.updated);
    // graphic_item_data
    //     .values_mut()
    //     .for_each(|data| data.updated = false);
}

fn update_graphic_item(
    item: &GraphicItem,
    batcher: &mut StyleBatcher,
    graphic_item_data: &mut HashMap<Uuid, GraphicItemData>,
    resolver: &StyleResolver,
    _model: &Model,
) {
    match item {
        GraphicItem::Cell(cell) => {
            let data = graphic_item_data
                .entry(cell.id)
                .or_insert_with(|| GraphicItemData::Cell {
                    cell_id: CellId::new(),
                    updated: false,
                });
            if let GraphicItemData::Cell { cell_id, updated } = data {
                batcher.add(&cell_id, resolver.cell(&cell.cell));
                *updated = true;
            }
        }
        GraphicItem::ClipArea(clip_area) => {
            let data = graphic_item_data.entry(clip_area.id).or_insert_with(|| {
                GraphicItemData::ClipArea {
                    cell_id: CellId::new(),
                    updated: false,
                    data: HashMap::new(),
                }
            });
            if let GraphicItemData::ClipArea {
                cell_id,
                updated,
                data: graphic_item_data,
            } = data
            {
                *updated = true;
                let clip_area_style = resolver.clip_area(&clip_area.clip_area);
                let new_resolver = resolver
                    .clone()
                    .with_position(clip_area_style.pos)
                    .with_render_layer(clip_area_style.render_layer);
                batcher.add_clip_area(&cell_id, clip_area_style);
                for item in clip_area.items.iter() {
                    update_graphic_item(item, batcher, graphic_item_data, &new_resolver, _model);
                }
            }
        }
        GraphicItem::DriverTable(driver_table) => {
            let data = graphic_item_data.entry(driver_table.id).or_insert_with(|| {
                GraphicItemData::DriverTable {
                    updated: false,
                    rows: HashMap::new(),
                    scroll_position: 0.0,
                }
            });
            if let GraphicItemData::DriverTable {
                updated,
                rows,
                scroll_position,
            } = data
            {
                *updated = true;

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
                let mut entries: Vec<&Entry> = resolver.session.entries.values().collect();
                entries.sort_by(|e1, e2| {
                    let is_connected = e2.connected.cmp(&e1.connected);
                    let position = e1
                        .position
                        .partial_cmp(&e2.position)
                        .unwrap_or(std::cmp::Ordering::Equal);
                    is_connected.then(position)
                });

                // Move rows to make sure the focused entry is visible
                if let Some(focused_entry_index) = entries.iter().position(|entry| entry.focused) {
                    let rows_to_skip = (focused_entry_index as f32 - 12.0)
                        .min(entries.len() as f32 - 23.0)
                        .max(0.0);
                    *scroll_position = *scroll_position - (*scroll_position - rows_to_skip) * 0.2;
                }

                for (index, entry) in entries.iter().enumerate() {
                    let data = rows.entry(entry.id).or_insert(HashMap::new());

                    let new_resolver = resolver
                        .clone()
                        .with_position(
                            resolver.position
                                - row_offset * *scroll_position
                                + row_offset * index as f32,
                        )
                        .with_entry(entry);

                    for column in driver_table.columns.iter() {
                        update_graphic_item(column, batcher, data, &new_resolver, _model);
                    }
                }
            }
        }
    }
}

#[derive(Clone)]
struct StyleResolver<'a> {
    value_store: &'a ValueStore,
    session: &'a Session,
    entry: Option<&'a Entry>,
    position: Vec3,
    render_layer: u8,
}
impl<'a> StyleResolver<'a> {
    fn new(value_store: &'a ValueStore, session: &'a Session) -> Self {
        Self {
            value_store,
            session,
            entry: None,
            position: Vec3::ZERO,
            render_layer: 0,
        }
    }

    fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }

    fn with_position(mut self, position: Vec3) -> Self {
        self.position = position;
        self
    }

    fn with_render_layer(mut self, render_layer: u8) -> Self {
        self.render_layer = render_layer;
        self
    }

    fn with_entry(mut self, entry: &'a Entry) -> Self {
        self.entry = Some(entry);
        self
    }

    fn property<T>(&self, property: &Property<T>) -> Option<T>
    where
        ValueStore: TypedValueResolver<T>,
        T: Clone,
    {
        self.value_store.get_property(property, self.entry)
    }

    fn clip_area(&self, clip_area: &ClipArea) -> ClipAreaStyle {
        ClipAreaStyle {
            pos: Vec3::new(
                self.value_store
                    .get_property(&clip_area.pos.x, self.entry)
                    .unwrap_or_default()
                    .0,
                self.value_store
                    .get_property(&clip_area.pos.y, self.entry)
                    .unwrap_or_default()
                    .0
                    * -1.0,
                self.value_store
                    .get_property(&clip_area.pos.z, self.entry)
                    .unwrap_or_default()
                    .0,
            ) + self.position,
            size: Vec2::new(
                self.value_store
                    .get_property(&clip_area.size.x, self.entry)
                    .unwrap_or_default()
                    .0,
                self.value_store
                    .get_property(&clip_area.size.y, self.entry)
                    .unwrap_or_default()
                    .0,
            ),
            corner_offsets: {
                let skew = self
                    .value_store
                    .get_property(&clip_area.skew, self.entry)
                    .unwrap_or_default()
                    .0;
                [
                    vec2(skew, 0.0),
                    vec2(skew, 0.0),
                    vec2(0.0, 0.0),
                    vec2(0.0, 0.0),
                ]
            },
            rounding: [
                self.value_store
                    .get_property(&clip_area.rounding.top_left, self.entry)
                    .unwrap_or(Number(0.0))
                    .0,
                self.value_store
                    .get_property(&clip_area.rounding.top_right, self.entry)
                    .unwrap_or(Number(0.0))
                    .0,
                self.value_store
                    .get_property(&clip_area.rounding.bot_left, self.entry)
                    .unwrap_or(Number(0.0))
                    .0,
                self.value_store
                    .get_property(&clip_area.rounding.bot_right, self.entry)
                    .unwrap_or(Number(0.0))
                    .0,
            ],
            render_layer: clip_area.render_layer,
        }
    }

    fn cell(&self, cell: &Cell) -> CellStyle {
        CellStyle {
            text: self
                .value_store
                .get_property(&cell.text, self.entry)
                .unwrap_or_else(|| Text("unavailable".to_string()))
                .0,
            text_color: self
                .value_store
                .get_property(&cell.text_color, self.entry)
                .unwrap_or(Tint(Color::BLACK))
                .0,
            text_size: self
                .value_store
                .get_property(&cell.text_size, self.entry)
                .unwrap_or(Number(20.0))
                .0,
            text_alignment: cell.text_alginment.clone(),
            text_position: Vec2::new(
                self.value_store
                    .get_property(&cell.text_position.x, self.entry)
                    .unwrap_or(Number(0.0))
                    .0,
                self.value_store
                    .get_property(&cell.text_position.y, self.entry)
                    .unwrap_or(Number(0.0))
                    .0,
            ),
            font: self
                .value_store
                .get_property(&cell.font, self.entry)
                .and_then(|f| match f {
                    Font::Default => None,
                    Font::Handle(handle) => Some(handle),
                }),
            color: self
                .value_store
                .get_property(&cell.color, self.entry)
                .unwrap_or(Tint(Color::RED))
                .0,
            texture: self
                .value_store
                .get_property(&cell.image, self.entry)
                .and_then(|t| match t {
                    Texture::None => None,
                    Texture::Handle(handle) => Some(handle),
                }),
            pos: Vec3::new(
                self.value_store
                    .get_property(&cell.pos.x, self.entry)
                    .unwrap_or(Number(0.0))
                    .0,
                self.value_store
                    .get_property(&cell.pos.y, self.entry)
                    .unwrap_or(Number(0.0))
                    .0
                    * -1.0,
                self.value_store
                    .get_property(&cell.pos.z, self.entry)
                    .unwrap_or(Number(0.0))
                    .0,
            ) + self.position,
            size: Vec2::new(
                self.value_store
                    .get_property(&cell.size.x, self.entry)
                    .unwrap_or(Number(0.0))
                    .0,
                self.value_store
                    .get_property(&cell.size.y, self.entry)
                    .unwrap_or(Number(0.0))
                    .0,
            ),
            corner_offsets: {
                let skew = self
                    .value_store
                    .get_property(&cell.skew, self.entry)
                    .unwrap_or_default()
                    .0;
                let get_vec = |prop: &Vec2Property| {
                    vec2(
                        self.value_store
                            .get_property(&prop.x, self.entry)
                            .unwrap_or_default()
                            .0,
                        -self
                            .value_store
                            .get_property(&prop.y, self.entry)
                            .unwrap_or_default()
                            .0,
                    )
                };
                [
                    get_vec(&cell.corner_offsets.top_left) + vec2(skew, 0.0),
                    get_vec(&cell.corner_offsets.top_right) + vec2(skew, 0.0),
                    get_vec(&cell.corner_offsets.bot_left) + vec2(0.0, 0.0),
                    get_vec(&cell.corner_offsets.bot_right) + vec2(0.0, 0.0),
                ]
            },
            visible: self
                .value_store
                .get_property(&cell.visible, self.entry)
                .unwrap_or(Boolean(true))
                .0,
            rounding: [
                self.value_store
                    .get_property(&cell.rounding.top_left, self.entry)
                    .unwrap_or(Number(0.0))
                    .0,
                self.value_store
                    .get_property(&cell.rounding.top_right, self.entry)
                    .unwrap_or(Number(0.0))
                    .0,
                self.value_store
                    .get_property(&cell.rounding.bot_left, self.entry)
                    .unwrap_or(Number(0.0))
                    .0,
                self.value_store
                    .get_property(&cell.rounding.bot_right, self.entry)
                    .unwrap_or(Number(0.0))
                    .0,
            ],
            render_layer: self.render_layer,
        }
    }
}
