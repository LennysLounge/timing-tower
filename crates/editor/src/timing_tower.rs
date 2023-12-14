use std::collections::HashMap;

use backend::{
    savefile::Savefile,
    style::{cell::Cell, StyleDefinition},
    style_batcher::{CellId, StyleBatcher},
    value_store::ValueStore,
    value_types::{Boolean, Number, Text, Texture, Tint},
};
use bevy::{
    ecs::system::{EntityCommand, ResMut},
    prelude::{
        Bundle, Color, Component, EntityWorldMut, Plugin, Query, Res, SpatialBundle, Update, Vec2,
        Vec3,
    },
};
use common::communication::CellStyle;
use unified_sim_model::{
    model::{Entry, EntryId, Session},
    Adapter,
};

pub struct TimingTowerPlugin;
impl Plugin for TimingTowerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, update_tower);
    }
}

#[derive(Bundle)]
pub struct TimingTowerBundle {
    pub spatial: SpatialBundle,
    pub tower_def: TimingTower,
}

#[derive(Component)]
pub struct TimingTower {
    pub cell_id: CellId,
    pub adapter: Adapter,
    pub table: Table,
}

pub struct Table {
    pub cell_id: CellId,
    pub rows: HashMap<EntryId, Row>,
}

pub struct Row {
    pub cell_id: CellId,
    pub columns: HashMap<String, CellId>,
}

pub fn init_timing_tower(adapter: Adapter) -> impl EntityCommand {
    |mut entity: EntityWorldMut| {
        entity.insert(TimingTower {
            cell_id: CellId::new(),
            adapter,
            table: Table {
                cell_id: CellId::new(),
                rows: HashMap::new(),
            },
        });
    }
}

pub fn update_tower(
    variables: Res<ValueStore>,
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

        let style = create_cell_style(&style_def.timing_tower.cell, &variables, Some(entry));
        style_batcher.add(&cell_id, style);

        // Update table
        let mut style =
            create_cell_style(&style_def.timing_tower.table.cell, &variables, Some(entry));
        style.pos.z += 1.0;
        style_batcher.add(&table.cell_id, style);

        update_rows(
            table,
            current_session,
            style_def,
            value_store.as_ref(),
            style_batcher.as_mut(),
        );

        // update columns
        for (entry_id, row) in table.rows.iter() {
            let Some(entry) = current_session.entries.get(entry_id) else {
                continue;
            };
            for column in style_def.timing_tower.table.row.all_columns() {
                let Some(cell_id) = row.columns.get(&column.name) else {
                    continue;
                };

                let mut style = create_cell_style(&column.cell, &variables, Some(entry));
                style.pos += Vec3::new(0.0, 0.0, 1.0);
                style_batcher.add(cell_id, style);
            }
        }
    }
}

fn update_rows(
    table: &mut Table,
    current_session: &Session,
    style: &StyleDefinition,
    value_store: &ValueStore,
    style_batcher: &mut StyleBatcher,
) {
    // Create rows for each entry
    for entry_id in current_session.entries.keys() {
        if !table.rows.contains_key(entry_id) {
            // create all necessairy cells for rows.
            let columns: HashMap<String, CellId> = style
                .timing_tower
                .table
                .row
                .all_columns()
                .iter()
                .map(|c| (c.name.clone(), CellId::new()))
                .collect();
            let row = Row {
                columns,
                cell_id: CellId::new(),
            };
            table.rows.insert(entry_id.clone(), row);
        }
    }

    // Update the rows
    let mut entries: Vec<&Entry> = current_session.entries.values().collect();
    entries.sort_by(|e1, e2| {
        let is_connected = e2.connected.cmp(&e1.connected);
        let position = e1
            .position
            .partial_cmp(&e2.position)
            .unwrap_or(std::cmp::Ordering::Equal);
        is_connected.then(position)
    });

    let mut offset = Vec2::new(0.0, 0.0);
    for entry in entries {
        let Some(row) = table.rows.get(&entry.id) else {
            continue;
        };

        let mut cell_style = create_cell_style(
            &style.timing_tower.table.row.cell,
            &value_store,
            Some(entry),
        );
        cell_style.pos += Vec3::new(offset.x, offset.y, 1.0);
        let row_height = cell_style.size.y;

        style_batcher.add(&row.cell_id, cell_style);

        offset.y -= row_height;
        offset -= Vec2::new(
            value_store
                .get_property(&style.timing_tower.table.row_offset.x, None)
                .unwrap_or(Number(0.0))
                .0
                * -1.0,
            value_store
                .get_property(&style.timing_tower.table.row_offset.y, None)
                .unwrap_or(Number(0.0))
                .0,
        );
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
            backend::style::cell::TextAlignment::Left => common::communication::TextAlignment::Left,
            backend::style::cell::TextAlignment::Center => {
                common::communication::TextAlignment::Center
            }
            backend::style::cell::TextAlignment::Right => {
                common::communication::TextAlignment::Right
            }
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
