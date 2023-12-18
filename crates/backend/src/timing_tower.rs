use std::collections::HashMap;

use crate::style;

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
use common::communication::CellStyle;
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
    pub adapter: Adapter,
    pub table: Table,
}
impl TimingTower {
    pub fn new(adapter: Adapter) -> Self {
        Self {
            cell_id: CellId::new(),
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
        };

        let (cell_style, row_resolver) =
            style_resolver.get_and_child(&style_def.scene.timing_tower.cell);
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
            .property(&style.row.row_offset.x)
            .unwrap_or_default()
            .0,
        -row_resolver
            .property(&style.row.row_offset.y)
            .unwrap_or_default()
            .0,
        0.0,
    );
    for entry in entries {
        let Some(row) = table.rows.get(&entry.id) else {
            continue;
        };

        row_resolver.entry = Some(entry);
        let (row_style, column_resolver) = row_resolver.get_and_child(&style.row.cell);
        row_resolver.position += row_offset + vec3(0.0, -row_style.size.y, 0.0);
        style_batcher.add(&row.cell_id, row_style);

        // update columns
        for column in style.row.contained_columns() {
            let Some(cell_id) = row.columns.get(&column.id) else {
                continue;
            };
            style_batcher.add(cell_id, column_resolver.get(&column.cell));
        }
    }
}

struct StyleResolver<'a> {
    value_store: &'a ValueStore,
    session: &'a Session,
    entry: Option<&'a Entry>,
    position: Vec3,
}
impl<'a> StyleResolver<'a> {
    fn get(&self, cell: &Cell) -> CellStyle {
        let mut style = create_cell_style(cell, self.value_store, self.entry);
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

    fn get_and_child(&self, cell: &Cell) -> (CellStyle, StyleResolver<'a>) {
        let style = self.get(cell);
        let resolver = StyleResolver {
            value_store: self.value_store,
            session: self.session,
            entry: self.entry,
            position: style.pos + vec3(0.0, 0.0, 1.0),
        };
        (style, resolver)
    }
}

fn create_cell_style(cell: &Cell, vars: &ValueStore, entry: Option<&Entry>) -> CellStyle {
    CellStyle {
        text: vars
            .get_property(&cell.text, entry)
            .unwrap_or_else(|| Text("unavailable".to_string()))
            .0,
        text_color: vars
            .get_property(&cell.text_color, entry)
            .unwrap_or(Tint(Color::BLACK))
            .0,
        text_size: vars
            .get_property(&cell.text_size, entry)
            .unwrap_or(Number(20.0))
            .0,
        text_alignment: match cell.text_alginment {
            crate::style::cell::TextAlignment::Left => common::communication::TextAlignment::Left,
            crate::style::cell::TextAlignment::Center => {
                common::communication::TextAlignment::Center
            }
            crate::style::cell::TextAlignment::Right => common::communication::TextAlignment::Right,
        },
        text_position: Vec2::new(
            vars.get_property(&cell.text_position.x, entry)
                .unwrap_or(Number(0.0))
                .0,
            vars.get_property(&cell.text_position.y, entry)
                .unwrap_or(Number(0.0))
                .0,
        ),
        color: vars
            .get_property(&cell.color, entry)
            .unwrap_or(Tint(Color::RED))
            .0,
        texture: vars.get_property(&cell.image, entry).and_then(|t| match t {
            Texture::None => None,
            Texture::Handle(handle) => Some(handle),
        }),
        pos: Vec3::new(
            vars.get_property(&cell.pos.x, entry)
                .unwrap_or(Number(0.0))
                .0,
            vars.get_property(&cell.pos.y, entry)
                .unwrap_or(Number(0.0))
                .0
                * -1.0,
            vars.get_property(&cell.pos.z, entry)
                .unwrap_or(Number(0.0))
                .0,
        ),
        size: Vec2::new(
            vars.get_property(&cell.size.x, entry)
                .unwrap_or(Number(0.0))
                .0,
            vars.get_property(&cell.size.y, entry)
                .unwrap_or(Number(0.0))
                .0,
        ),
        skew: vars
            .get_property(&cell.skew, entry)
            .unwrap_or(Number(0.0))
            .0,
        visible: vars
            .get_property(&cell.visible, entry)
            .unwrap_or(Boolean(true))
            .0,
        rounding: [
            vars.get_property(&cell.rounding.top_left, entry)
                .unwrap_or(Number(0.0))
                .0,
            vars.get_property(&cell.rounding.top_right, entry)
                .unwrap_or(Number(0.0))
                .0,
            vars.get_property(&cell.rounding.bot_right, entry)
                .unwrap_or(Number(0.0))
                .0,
            vars.get_property(&cell.rounding.bot_left, entry)
                .unwrap_or(Number(0.0))
                .0,
        ],
    }
}
