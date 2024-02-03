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
    math::{vec2, Vec2, Vec3},
    render::color::Color,
    utils::hashbrown::HashSet,
};
use common::communication::{CellStyle, ClipAreaStyle};
use unified_sim_model::model::{Entry, Model, Session};
use uuid::Uuid;

use crate::{
    savefile::Savefile,
    style::{
        self,
        cell::{Cell, ClipArea},
        elements::GraphicItem,
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

struct GraphicItemData {
    cell_id: CellId,
    updated: bool,
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
                let resolver = StyleResolver::new(&*value_store, session);
                update_graphic_items(
                    &graphic.items.items,
                    &mut batcher,
                    &mut graphic_item_data,
                    resolver,
                    &*model,
                );
            }
        });
    }

    // Remove stale data
    graphic_item_data.retain(|_, data| data.updated);
    graphic_item_data
        .values_mut()
        .for_each(|data| data.updated = false);
}

fn update_graphic_items(
    items: &Vec<GraphicItem>,
    batcher: &mut StyleBatcher,
    graphic_item_data: &mut HashMap<Uuid, GraphicItemData>,
    style_resolver: StyleResolver,
    _model: &Model,
) {
    for graphic_item in items.iter() {
        match graphic_item {
            style::elements::GraphicItem::Cell(cell) => {
                let data = graphic_item_data
                    .entry(cell.id)
                    .or_insert_with(|| GraphicItemData {
                        cell_id: CellId::new(),
                        updated: false,
                    });

                batcher.add(&data.cell_id, style_resolver.cell(&cell.cell));
                data.updated = true;
            }
            style::elements::GraphicItem::ClipArea(_) => (),
        }
    }
}

struct StyleResolver<'a> {
    value_store: &'a ValueStore,
    _session: &'a Session,
    entry: Option<&'a Entry>,
    position: Vec3,
    render_layer: u8,
}
impl<'a> StyleResolver<'a> {
    fn new(value_store: &'a ValueStore, session: &'a Session) -> Self {
        Self {
            value_store,
            _session: session,
            entry: None,
            position: Vec3::ZERO,
            render_layer: 0,
        }
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
