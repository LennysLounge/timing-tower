use std::collections::HashMap;

use backend::{
    savefile::Savefile,
    style::cell::Cell,
    value_store::ValueStore,
    value_types::{Boolean, Number, Text, Texture, Tint},
};
use bevy::{
    ecs::{schedule::IntoSystemConfigs, system::EntityCommand},
    prelude::{
        BuildChildren, BuildWorldChildren, Bundle, Color, Commands, Component, Entity,
        EntityWorldMut, EventWriter, Plugin, Query, Res, SpatialBundle, Update, Vec2, Vec3, With,
    },
};
use common::cell::{
    init_cell,
    style::{CellStyle, SetStyle},
};
use unified_sim_model::{
    model::{Entry, EntryId},
    Adapter,
};

use crate::SpawnAndInitWorld;

pub struct TimingTowerPlugin;
impl Plugin for TimingTowerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (update_tower, update_table, update_rows, update_columns).chain(),
        );
    }
}

#[derive(Bundle)]
pub struct TimingTowerBundle {
    pub spatial: SpatialBundle,
    pub tower_def: TimingTower,
}

#[derive(Component)]
pub struct TimingTower {
    pub adapter: Adapter,
    pub table_id: Entity,
}

#[derive(Component)]
pub struct Table {
    pub tower_id: Entity,
    pub rows: HashMap<EntryId, Entity>,
}

#[derive(Component)]
pub struct Row {
    pub tower_id: Entity,
    pub entry_id: EntryId,
    pub columns: HashMap<String, Entity>,
}

#[derive(Component)]
struct LogPosition(Vec3);

pub fn init_timing_tower(adapter: Adapter) -> impl EntityCommand {
    |mut entity: EntityWorldMut| {
        let tower_id = entity.id();
        let table_id = entity.world_scope(|world| {
            world
                .spawn_new(init_cell)
                .insert(Table {
                    tower_id,
                    rows: HashMap::new(),
                })
                .id()
        });

        entity.world_scope(|world| {
            init_cell(world.entity_mut(tower_id));
        });

        entity
            .insert(TimingTower { adapter, table_id })
            .insert(LogPosition(Vec3::ZERO))
            .add_child(table_id);
    }
}

pub fn update_tower(
    variables: Res<ValueStore>,
    savefile: Option<Res<Savefile>>,
    mut towers: Query<(Entity, &TimingTower), With<TimingTower>>,
    mut set_style_event: EventWriter<SetStyle>,
) {
    let Some(style) = savefile.as_ref().map(|s| s.style()) else {
        return;
    };

    for (tower_id, tower) in towers.iter_mut() {
        let Ok(model) = tower.adapter.model.read() else {
            continue;
        };

        let Some(current_session) = model.current_session() else {
            continue;
        };

        let Some(entry) = current_session.entries.values().next() else {
            continue;
        };

        let style = create_cell_style(&style.timing_tower.cell, &variables, Some(entry));
        set_style_event.send(SetStyle {
            entity: tower_id,
            style,
        });
    }
}

fn update_table(
    tables: Query<(Entity, &Table)>,
    towers: Query<&TimingTower>,
    savefile: Option<Res<Savefile>>,
    variables: Res<ValueStore>,
    mut set_style_event: EventWriter<SetStyle>,
) {
    let Some(style) = savefile.as_ref().map(|s| s.style()) else {
        return;
    };

    for (table_id, table) in tables.iter() {
        let Ok(tower) = towers.get(table.tower_id) else {
            continue;
        };

        let Ok(model) = tower.adapter.model.read() else {
            continue;
        };

        let Some(current_session) = model.current_session() else {
            continue;
        };

        let Some(entry) = current_session.entries.values().next() else {
            continue;
        };

        let mut style = create_cell_style(&style.timing_tower.table.cell, &variables, Some(entry));
        style.pos.z += 1.0;
        set_style_event.send(SetStyle {
            entity: table_id.clone(),
            style,
        });
    }
}

fn update_rows(
    towers: Query<&TimingTower>,
    savefile: Option<Res<Savefile>>,
    variables: Res<ValueStore>,
    mut tables: Query<(Entity, &mut Table)>,
    mut commands: Commands,
    mut set_style_event: EventWriter<SetStyle>,
) {
    let Some(style) = savefile.as_ref().map(|s| s.style()) else {
        return;
    };
    for (table_id, mut table) in tables.iter_mut() {
        let Ok(tower) = towers.get(table.tower_id) else {
            continue;
        };

        let Ok(model) = tower.adapter.model.read() else {
            continue;
        };

        let Some(current_session) = model.current_session() else {
            continue;
        };

        // Create rows for each entry
        for entry_id in current_session.entries.keys() {
            if !table.rows.contains_key(entry_id) {
                let row_id = commands.spawn_empty().add(init_cell).id();
                // create all necessairy cells for rows.
                let mut columns = HashMap::new();
                for column in style.timing_tower.table.row.columns.all_t() {
                    let cell_id = commands.spawn_empty().add(init_cell).id();
                    columns.insert(column.name.clone(), cell_id);
                    commands.entity(row_id).add_child(cell_id);
                }
                commands.entity(row_id).insert(Row {
                    tower_id: table.tower_id,
                    entry_id: *entry_id,
                    columns,
                });
                table.rows.insert(entry_id.clone(), row_id);
                // add row as child to table.
                commands.entity(table_id).add_child(row_id);
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
            let Some(row_id) = table.rows.get(&entry.id) else {
                continue;
            };

            let mut cell_style =
                create_cell_style(&style.timing_tower.table.row.cell, &variables, Some(entry));
            cell_style.pos += Vec3::new(offset.x, offset.y, 1.0);
            let row_height = cell_style.size.y;
            set_style_event.send(SetStyle {
                entity: *row_id,
                style: cell_style,
            });

            offset.y -= row_height;
            offset -= Vec2::new(
                variables
                    .get_property(&style.timing_tower.table.row_offset.x, None)
                    .unwrap_or(Number(0.0))
                    .0
                    * -1.0,
                variables
                    .get_property(&style.timing_tower.table.row_offset.y, None)
                    .unwrap_or(Number(0.0))
                    .0,
            );
        }
    }
}

fn update_columns(
    rows: Query<&Row>,
    towers: Query<&TimingTower>,
    savefile: Option<Res<Savefile>>,
    variables: Res<ValueStore>,
    mut set_style_event: EventWriter<SetStyle>,
) {
    let Some(style) = savefile.as_ref().map(|s| s.style()) else {
        return;
    };
    for row in rows.iter() {
        let Ok(tower) = towers.get(row.tower_id) else {
            continue;
        };

        let Ok(model) = tower.adapter.model.read() else {
            continue;
        };

        let Some(current_session) = model.current_session() else {
            continue;
        };

        let Some(entry) = current_session.entries.get(&row.entry_id) else {
            continue;
        };

        for column in style.timing_tower.table.row.columns.all_t() {
            let Some(cell_id) = row.columns.get(&column.name) else {
                continue;
            };

            let mut style = create_cell_style(&column.cell, &variables, Some(entry));
            style.pos += Vec3::new(0.0, 0.0, 1.0);
            set_style_event.send(SetStyle {
                entity: cell_id.clone(),
                style,
            });
        }
    }
}

fn create_cell_style(cell: &Cell, vars: &ValueStore, entry: Option<&Entry>) -> CellStyle {
    // let text = match &cell.value_source {
    //     ValueSource::FixedValue(s) => s.clone(),
    //     ValueSource::DriverName => {
    //         let driver = entry.drivers.get(&entry.current_driver).and_then(|d| {
    //             let letter = if d.first_name.len() > 0 {
    //                 &d.first_name[0..1]
    //             } else {
    //                 ""
    //             };
    //             Some(format!("{}. {}", letter, d.last_name))
    //         });
    //         driver.unwrap_or_else(|| "no driver".to_string())
    //     }
    //     ValueSource::Position => format!("{}", entry.position),
    //     ValueSource::CarNumber => format!("{}", entry.car_number),
    // };

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
            backend::style::cell::TextAlignment::Left => common::cell::style::TextAlignment::Left,
            backend::style::cell::TextAlignment::Center => {
                common::cell::style::TextAlignment::Center
            }
            backend::style::cell::TextAlignment::Right => common::cell::style::TextAlignment::Right,
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
