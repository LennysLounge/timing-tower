use std::collections::HashMap;

use bevy::{
    ecs::system::EntityCommand,
    prelude::{
        BuildChildren, BuildWorldChildren, Bundle, Camera, Color, Commands, Component, Entity,
        EventWriter, GlobalTransform, IntoSystemConfigs, Plugin, Query, Res, SpatialBundle, Update,
        Vec2, Vec3, With, World,
    },
};
use unified_sim_model::{
    model::{Entry, EntryId},
    Adapter,
};

use crate::{
    cell::{init_cell, CellStyle, SetStyle},
    editor::style_elements::{CellElement, RootElement},
    variable_repo::VariableRepo,
    MainCamera, SpawnAndInitWorld,
};

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
    |tower_id: Entity, world: &mut World| {
        let table_id = world
            .spawn_new(init_cell)
            .insert(Table {
                tower_id,
                rows: HashMap::new(),
            })
            .id();

        init_cell(tower_id, world);
        world
            .entity_mut(tower_id)
            .insert(TimingTower { adapter, table_id })
            .insert(LogPosition(Vec3::ZERO))
            .add_child(table_id);
    }
}

pub fn update_tower(
    main_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    elements: Res<RootElement>,
    variables: Res<VariableRepo>,
    mut towers: Query<(Entity, &TimingTower), With<TimingTower>>,
    mut set_style_event: EventWriter<SetStyle>,
) {
    let (camera, camera_transform) = main_camera.single();
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

        let mut style = create_cell_style(&elements.timing_tower.cell, &variables, Some(entry));
        // The cell position is relative to its parent. The timing tower itself doesnt
        // have a parent so this needs to be added to get it into the right position.
        let top_left = camera
            .viewport_to_world_2d(camera_transform, Vec2::new(0.0, 0.0))
            .unwrap_or(Vec2::ZERO);
        style.pos += Vec3::new(top_left.x, top_left.y, 0.0);
        set_style_event.send(SetStyle {
            entity: tower_id,
            style,
        });
    }
}

fn update_table(
    tables: Query<(Entity, &Table)>,
    towers: Query<&TimingTower>,
    elements: Res<RootElement>,
    variables: Res<VariableRepo>,
    mut set_style_event: EventWriter<SetStyle>,
) {
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

        let mut style =
            create_cell_style(&elements.timing_tower.table.cell, &variables, Some(entry));
        style.pos.z += 1.0;
        set_style_event.send(SetStyle {
            entity: table_id.clone(),
            style,
        });
    }
}

fn update_rows(
    towers: Query<&TimingTower>,
    elements: Res<RootElement>,
    variables: Res<VariableRepo>,
    mut tables: Query<(Entity, &mut Table)>,
    mut commands: Commands,
    mut set_style_event: EventWriter<SetStyle>,
) {
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
                for column in elements.timing_tower.table.row.columns.iter() {
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

            let mut style = create_cell_style(
                &elements.timing_tower.table.row.cell,
                &variables,
                Some(entry),
            );
            style.pos += Vec3::new(offset.x, offset.y, 1.0);
            let row_height = style.size.y;
            set_style_event.send(SetStyle {
                entity: *row_id,
                style,
            });

            offset.y -= row_height;
            offset -= Vec2::new(
                variables
                    .get_number(&elements.timing_tower.table.row_offset.x, None)
                    .unwrap_or(0.0)
                    * -1.0,
                variables
                    .get_number(&elements.timing_tower.table.row_offset.y, None)
                    .unwrap_or(0.0),
            );
        }
    }
}

fn update_columns(
    rows: Query<&Row>,
    towers: Query<&TimingTower>,
    elements: Res<RootElement>,
    variables: Res<VariableRepo>,
    mut set_style_event: EventWriter<SetStyle>,
) {
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

        for column in elements.timing_tower.table.row.columns.iter() {
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

fn create_cell_style(cell: &CellElement, vars: &VariableRepo, entry: Option<&Entry>) -> CellStyle {
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
            .get_text(&cell.value_source, entry)
            .unwrap_or_else(|| "unavailable".to_string()),
        text_alignment: cell.text_alginment.clone(),
        text_position: Vec2::new(
            vars.get_number(&cell.text_position.x, entry).unwrap_or(0.0),
            vars.get_number(&cell.text_position.y, entry).unwrap_or(0.0),
        ),
        color: vars.get_color(&cell.color, entry).unwrap_or(Color::RED),
        texture: None,
        pos: Vec3::new(
            vars.get_number(&cell.pos.x, entry).unwrap_or(0.0),
            vars.get_number(&cell.pos.y, entry).unwrap_or(0.0) * -1.0,
            vars.get_number(&cell.pos.z, entry).unwrap_or(0.0),
        ),
        size: Vec2::new(
            vars.get_number(&cell.size.x, entry).unwrap_or(0.0),
            vars.get_number(&cell.size.y, entry).unwrap_or(0.0),
        ),
        skew: vars.get_number(&cell.skew, entry).unwrap_or(0.0),
        visible: cell.visible,
        rounding: [
            vars.get_number(&cell.rounding.top_left, entry)
                .unwrap_or(0.0),
            vars.get_number(&cell.rounding.top_right, entry)
                .unwrap_or(0.0),
            vars.get_number(&cell.rounding.bot_right, entry)
                .unwrap_or(0.0),
            vars.get_number(&cell.rounding.bot_left, entry)
                .unwrap_or(0.0),
        ],
    }
}
