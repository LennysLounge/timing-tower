use std::f32::consts::PI;

use bevy::{
    prelude::{
        shape, AssetServer, Assets, BuildChildren, Color, Commands, Component, Entity, EventReader,
        Handle, Mesh, Plugin, PostUpdate, Query, Res, ResMut, Update, Vec2, Vec3, With,
    },
    render::{mesh::Indices, primitives::Aabb},
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::gradient_material::{Gradient, GradientMaterial};

use super::{CellStyle, SetStyle};

pub struct BackgroundPlugin;
impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, add_background)
            .add_systems(PostUpdate, update_style);
    }
}

#[derive(Component, Default)]
pub struct AddBackground;

#[derive(Component)]
pub struct Background(pub Entity);

fn add_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<GradientMaterial>>,
    entities: Query<Entity, With<AddBackground>>,
) {
    for entity in entities.iter() {
        let background = commands
            .spawn(MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::RegularPolygon::new(50.0, 80).into())
                    .into(),
                material: materials.add(GradientMaterial {
                    color: Color::PURPLE,
                    gradient: Gradient::None,
                    texture: None,
                }),
                ..Default::default()
            })
            .id();

        let mut entity = commands.entity(entity);
        entity.remove::<AddBackground>();
        entity.add_child(background);
        entity.insert(Background(background));
    }
}

fn update_style(
    mut events: EventReader<SetStyle>,
    mut materials_assets: ResMut<Assets<GradientMaterial>>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    cells: Query<&Background>,
    materials_handles: Query<&Handle<GradientMaterial>>,
    mut mesh_handles: Query<(&Mesh2dHandle, &mut Aabb)>,
    asset_server: Res<AssetServer>,
) {
    for event in events.iter() {
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
        material.texture = match &event.style.texture {
            Some(path) => Some(asset_server.load(path)),
            None => None,
        };

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
    // Corner of the box with skew applied
    let corners = vec![
        Vec3::new(style.skew, 0.0, 0.0),
        Vec3::new(style.size.x + style.skew, 0.0, 0.0),
        Vec3::new(style.size.x, -style.size.y, 0.0),
        Vec3::new(0.0, -style.size.y, 0.0),
    ];

    // To calculate the rounded corners we first have to trasnlate the rounding radius
    // into the distance 'l'. 'l' describes the distance from the corner to the point where
    // the circle touches the edge. For corner rounding with a circle, the distance 'l' is
    // the same for both edges.

    // Step skipped for simplicity
    let l = vec![
        style.rounding[0],
        style.rounding[1],
        style.rounding[2],
        style.rounding[3],
    ];

    // Next we calculate the normalised vectors for the edges
    let edges = vec![
        (corners[0] - corners[3]).normalize(),
        (corners[1] - corners[0]).normalize(),
        (corners[2] - corners[1]).normalize(),
        (corners[3] - corners[2]).normalize(),
    ];

    // Now calculate the rounding for each corner
    let mut positions = Vec::new();
    for i in 0..4 {
        let steps = 50;
        for j in 0..steps {
            let angle = PI / (2.0 * (steps - 1) as f32) * j as f32;
            positions.push(
                corners[i] - edges[i] * l[i] * (1.0 - angle.sin())
                    + edges[(i + 1) % 4] * l[i] * (1.0 - angle.cos()),
            );
        }
    }

    let mut min = Vec2::new(f32::MAX, f32::MAX);
    let mut max = Vec2::new(f32::MIN, f32::MIN);
    for vertex in positions.iter() {
        min.x = min.x.min(vertex.x);
        min.y = min.y.min(vertex.y);
        max.x = max.x.max(vertex.x);
        max.y = max.y.max(vertex.y);
    }
    let width = max.x - min.x;
    let height = max.y - min.y;

    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    for vertex in positions.iter() {
        normals.push([0.0, 0.0, 1.0]);
        uvs.push([
            (vertex.x - min.x) / width,
            (vertex.y - max.y) / height * -1.0,
        ]);
    }

    let mut indices = Vec::new();
    for i in 1..(positions.len() - 1) {
        indices.extend([0, i as u32, i as u32 + 1]);
    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(Indices::U32(indices)));

    if let Some(new_aabb) = mesh.compute_aabb() {
        *aabb = new_aabb;
    }
}
