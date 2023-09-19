use bevy::{
    prelude::{
        shape, Assets, BuildChildren, Color, Commands, Component, Entity, EventReader, Handle,
        Mesh, Plugin, PostUpdate, Query, ResMut, Update, With,
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
    let positions = vec![
        [style.skew, 0.0, 0.0],
        [style.size.x + style.skew, 0.0, 0.0],
        [style.size.x, -style.size.y, 0.0],
        [0.0, -style.size.y, 0.0],
    ];
    let normals = vec![
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
    ];
    let uvs = vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
    let indices = vec![0, 1, 2, 0, 2, 3];

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(Indices::U32(indices)));

    if let Some(new_aabb) = mesh.compute_aabb() {
        *aabb = new_aabb;
    }
}
