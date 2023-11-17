use bevy::{
    math::{vec2, vec3},
    prelude::{Assets, Component, Entity, EventReader, Handle, Mesh, Query, ResMut},
    render::{mesh::Indices, primitives::Aabb},
    sprite::Mesh2dHandle,
};

use crate::cell_material::CellMaterial;

use super::{style::CellStyle, SetStyle};

#[derive(Component)]
pub struct Background(pub Entity);

pub fn update_style(
    mut events: EventReader<SetStyle>,
    mut materials_assets: ResMut<Assets<CellMaterial>>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    cells: Query<&Background>,
    materials_handles: Query<&Handle<CellMaterial>>,
    mut mesh_handles: Query<(&Mesh2dHandle, &mut Aabb)>,
) {
    for event in events.read() {
        let Ok(background_hadle) = cells.get(event.entity) else {
            continue;
        };
        let Ok(material_handle) = materials_handles.get(background_hadle.0) else {
            continue;
        };
        let Some(material) = materials_assets.get_mut(material_handle) else {
            continue;
        };

        material.color = event.style.color;
        material.texture = event.style.texture.clone();
        material.size = event.style.size;
        material.rounding = event.style.rounding.into();

        let Ok((mesh_handle, mut aabb)) = mesh_handles.get_mut(background_hadle.0) else {
            continue;
        };

        let Some(mesh) = mesh_assets.get_mut(&mesh_handle.0) else {
            continue;
        };
        update_mesh(&event.style, mesh, &mut aabb);
    }
}

fn update_mesh(style: &CellStyle, mesh: &mut Mesh, aabb: &mut Aabb) {
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            vec3(style.skew, 0.0, 0.0),
            vec3(style.size.x + style.skew, 0.0, 0.0),
            vec3(style.size.x, -style.size.y, 0.0),
            vec3(0.0, -style.size.y, 0.0),
        ],
    );
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vec![
            vec3(0.0, 0.0, 1.0),
            vec3(0.0, 0.0, 1.0),
            vec3(0.0, 0.0, 1.0),
            vec3(0.0, 0.0, 1.0),
        ],
    );
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![
            vec2(0.0, 0.0),
            vec2(1.0, 0.0),
            vec2(1.0, 1.0),
            vec2(0.0, 1.0),
        ],
    );
    mesh.set_indices(Some(Indices::U32(vec![0, 1, 2, 0, 2, 3])));

    if let Some(new_aabb) = mesh.compute_aabb() {
        *aabb = new_aabb;
    }
}
