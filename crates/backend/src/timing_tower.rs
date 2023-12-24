use std::collections::HashMap;

use crate::style::clip_area::ClipAreaData;

use super::{
    savefile::Savefile,
    style::cell::Cell,
    style_batcher::{CellId, StyleBatcher},
    value_store::{TypedValueResolver, ValueStore},
    value_types::{Boolean, Number, Property, Text, Texture, Tint},
};
use bevy::{
    ecs::system::ResMut,
    math::vec3,
    prelude::{Color, Component, Plugin, Query, Res, Update, Vec2, Vec3},
};
use common::communication::{CellStyle, ClipAreaStyle};
use unified_sim_model::{
    model::{Entry, EntryId, Session},
    Adapter,
};
use uuid::Uuid;

pub struct TimingTowerPlugin;
impl Plugin for TimingTowerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, update_tower);
    }
}

#[derive(Component)]
pub struct TimingTower {
    pub cell_id: CellId,
    pub clip_area_cell_id: CellId,
    pub adapter: Adapter,
    pub rows: HashMap<EntryId, Row>,
}
pub struct Row {
    pub cell_id: CellId,
    pub columns: HashMap<Uuid, CellId>,
}
impl TimingTower {
    pub fn new(adapter: Adapter) -> Self {
        Self {
            cell_id: CellId::new(),
            clip_area_cell_id: CellId::new(),
            adapter,
            rows: HashMap::new(),
        }
    }
}

pub fn update_tower(
    savefile: Option<Res<Savefile>>,
    value_store: Res<ValueStore>,
    mut towers: Query<&mut TimingTower>,
    mut batcher: ResMut<StyleBatcher>,
) {
    let Some(tower_style) = savefile.as_ref().map(|s| &s.style().scene.timing_tower) else {
        return;
    };

    for mut tower in towers.iter_mut() {
        let TimingTower {
            cell_id,
            clip_area_cell_id,
            adapter,
            rows,
        } = tower.as_mut();

        let Ok(model) = adapter.model.read() else {
            continue;
        };

        let Some(current_session) = model.current_session() else {
            continue;
        };

        let Some(entry) = current_session.entries.values().next() else {
            continue;
        };

        let resolver = StyleResolver {
            value_store: &*value_store,
            session: current_session,
            entry: Some(entry),
            position: Vec3::ZERO,
            render_layer: 0,
        };

        let cell = resolver.cell(&tower_style.cell);
        let row_resolver = resolver.with_position(cell.pos);

        let clip_area = row_resolver.clip_area(&tower_style.row.data);
        let mut row_resolver = row_resolver.with_render_layer(clip_area.render_layer);

        batcher.add(&cell_id, cell);
        batcher.add_clip_area(&clip_area_cell_id, clip_area);

        // Create rows for each new entry
        for entry_id in resolver.session.entries.keys() {
            rows.entry(*entry_id).or_insert(Row {
                cell_id: CellId::new(),
                columns: tower_style
                    .row
                    .inner
                    .contained_columns()
                    .iter()
                    .map(|c| (c.id, CellId::new()))
                    .collect(),
            });
        }

        // Update the rows
        let row_offset = vec3(
            row_resolver
                .property(&tower_style.row.inner.row_offset.x)
                .unwrap_or_default()
                .0,
            -row_resolver
                .property(&tower_style.row.inner.row_offset.y)
                .unwrap_or_default()
                .0,
            0.0,
        );

        // Update the rows
        let mut entries: Vec<&Entry> = row_resolver.session.entries.values().collect();
        entries.sort_by(|e1, e2| {
            let is_connected = e2.connected.cmp(&e1.connected);
            let position = e1
                .position
                .partial_cmp(&e2.position)
                .unwrap_or(std::cmp::Ordering::Equal);
            is_connected.then(position)
        });
        for entry in entries {
            let Some(row) = rows.get(&entry.id) else {
                continue;
            };

            row_resolver.entry = Some(entry);
            let cell = row_resolver.cell(&tower_style.row.inner.cell);

            let column_resolver = row_resolver.with_position(cell.pos);

            row_resolver.position += row_offset + vec3(0.0, -cell.size.y, 0.0);
            batcher.add(&row.cell_id, cell);

            // update columns
            for column in tower_style.row.inner.contained_columns() {
                let Some(cell_id) = row.columns.get(&column.id) else {
                    continue;
                };
                batcher.add(cell_id, column_resolver.cell(&column.cell));
            }
        }
    }
}

struct StyleResolver<'a> {
    value_store: &'a ValueStore,
    session: &'a Session,
    entry: Option<&'a Entry>,
    position: Vec3,
    render_layer: u8,
}
impl<'a> StyleResolver<'a> {
    fn property<T>(&self, property: &Property<T>) -> Option<T>
    where
        ValueStore: TypedValueResolver<T>,
        T: Clone,
    {
        self.value_store.get_property(property, self.entry)
    }

    fn with_position(&self, position: Vec3) -> Self {
        Self {
            position: position + vec3(0.0, 0.0, 1.0),
            ..*self
        }
    }

    fn with_render_layer(&self, layer: u8) -> Self {
        Self {
            render_layer: layer,
            ..*self
        }
    }

    fn clip_area(&self, clip_area: &ClipAreaData) -> ClipAreaStyle {
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
            skew: self
                .value_store
                .get_property(&clip_area.skew, self.entry)
                .unwrap_or_default()
                .0,
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
                    .get_property(&clip_area.rounding.bot_right, self.entry)
                    .unwrap_or(Number(0.0))
                    .0,
                self.value_store
                    .get_property(&clip_area.rounding.bot_left, self.entry)
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
            skew: self
                .value_store
                .get_property(&cell.skew, self.entry)
                .unwrap_or(Number(0.0))
                .0,
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
                    .get_property(&cell.rounding.bot_right, self.entry)
                    .unwrap_or(Number(0.0))
                    .0,
                self.value_store
                    .get_property(&cell.rounding.bot_left, self.entry)
                    .unwrap_or(Number(0.0))
                    .0,
            ],
            render_layer: self.render_layer,
        }
    }
}
