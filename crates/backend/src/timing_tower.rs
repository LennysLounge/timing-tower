use std::collections::HashMap;

use crate::style::{self, clip_area::ClipAreaData};

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
    pub table: Table,
}
impl TimingTower {
    pub fn new(adapter: Adapter) -> Self {
        Self {
            cell_id: CellId::new(),
            clip_area_cell_id: CellId::new(),
            adapter,
            table: Table {
                cell_id: CellId::new(),
                rows: HashMap::new(),
            },
        }
    }
}

pub struct Table {
    pub cell_id: CellId,
    pub rows: HashMap<EntryId, Row>,
}

pub struct Row {
    pub cell_id: CellId,
    pub columns: HashMap<Uuid, CellId>,
}

pub fn update_tower(
    savefile: Option<Res<Savefile>>,
    value_store: Res<ValueStore>,
    mut towers: Query<&mut TimingTower>,
    mut style_batcher: ResMut<StyleBatcher>,
) {
    let Some(style_def) = savefile.as_ref().map(|s| s.style()) else {
        return;
    };

    for mut tower in towers.iter_mut() {
        let TimingTower {
            cell_id,
            clip_area_cell_id,
            adapter,
            table,
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

        let style_resolver = StyleResolver {
            value_store: &*value_store,
            session: current_session,
            entry: Some(entry),
            position: Vec3::ZERO,
            render_layer: 0,
        };

        let clip_area = style_resolver.clip_area(&style_def.scene.timing_tower.row.data);
        style_batcher.add_clip_area(&clip_area_cell_id, clip_area);

        let (cell_style, row_resolver) =
            style_resolver.cell_and_child(&style_def.scene.timing_tower.cell);
        style_batcher.add(&cell_id, cell_style);

        // Update table
        update_row(
            table,
            &style_def.scene.timing_tower,
            row_resolver,
            style_batcher.as_mut(),
        );
    }
}

fn update_row(
    table: &mut Table,
    style: &style::timing_tower::TimingTower,
    mut row_resolver: StyleResolver<'_>,
    style_batcher: &mut StyleBatcher,
) {
    // let (table_style, mut row_resolver) = style_resolver.get_and_child(&style.table.cell);
    // style_batcher.add(&table.cell_id, table_style);

    // Create rows for each entry
    for entry_id in row_resolver.session.entries.keys() {
        if !table.rows.contains_key(entry_id) {
            // create all necessairy cells for rows.
            let columns: HashMap<Uuid, CellId> = style
                .row
                .inner
                .contained_columns()
                .iter()
                .map(|c| (c.id, CellId::new()))
                .collect();
            let row = Row {
                columns,
                cell_id: CellId::new(),
            };
            table.rows.insert(entry_id.clone(), row);
        }
    }

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

    let row_offset = vec3(
        row_resolver
            .property(&style.row.inner.row_offset.x)
            .unwrap_or_default()
            .0,
        -row_resolver
            .property(&style.row.inner.row_offset.y)
            .unwrap_or_default()
            .0,
        0.0,
    );
    for entry in entries {
        let Some(row) = table.rows.get(&entry.id) else {
            continue;
        };

        row_resolver.entry = Some(entry);
        let (row_style, column_resolver) = row_resolver.cell_and_child(&style.row.inner.cell);
        row_resolver.position += row_offset + vec3(0.0, -row_style.size.y, 0.0);
        style_batcher.add(&row.cell_id, row_style);

        // update columns
        for column in style.row.inner.contained_columns() {
            let Some(cell_id) = row.columns.get(&column.id) else {
                continue;
            };
            style_batcher.add(cell_id, column_resolver.cell(&column.cell));
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
    fn cell(&self, cell: &Cell) -> CellStyle {
        let mut style = self.create_cell_style(cell);
        style.pos += self.position;
        style
    }

    fn clip_area(&self, clip_area: &ClipAreaData) -> ClipAreaStyle {
        let mut style = self.create_clip_area_style(clip_area);
        style.pos += self.position;
        style
    }

    fn property<T>(&self, property: &Property<T>) -> Option<T>
    where
        ValueStore: TypedValueResolver<T>,
        T: Clone,
    {
        self.value_store.get_property(property, self.entry)
    }

    fn cell_and_child(&self, cell: &Cell) -> (CellStyle, StyleResolver<'a>) {
        let style = self.cell(cell);
        let resolver = StyleResolver {
            value_store: self.value_store,
            session: self.session,
            entry: self.entry,
            position: style.pos + vec3(0.0, 0.0, 1.0),
            render_layer: self.render_layer,
        };
        (style, resolver)
    }

    fn create_clip_area_style(&self, clip_area: &ClipAreaData) -> ClipAreaStyle {
        ClipAreaStyle {
            pos: Vec3::new(
                self.value_store
                    .get_property(&clip_area.pos.x, self.entry)
                    .unwrap_or_default()
                    .0,
                self.value_store
                    .get_property(&clip_area.pos.y, self.entry)
                    .unwrap_or_default()
                    .0,
                self.value_store
                    .get_property(&clip_area.pos.z, self.entry)
                    .unwrap_or_default()
                    .0,
            ),
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
            render_layer: self.render_layer,
        }
    }

    fn create_cell_style(&self, cell: &Cell) -> CellStyle {
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
            ),
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
