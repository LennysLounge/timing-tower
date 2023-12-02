use bevy::{
    ecs::system::Res,
    math::vec3,
    prelude::{Assets, Component, Entity, EventReader, Handle, Query, ResMut},
    render::primitives::Aabb,
};

use crate::{asset_store::AssetStore, cell_material::CellMaterial};

use super::SetStyle;

#[derive(Component)]
pub struct Background(pub Entity);

pub fn update_style(
    mut events: EventReader<SetStyle>,
    mut materials_assets: ResMut<Assets<CellMaterial>>,
    cells: Query<&Background>,
    mut background: Query<(&Handle<CellMaterial>, &mut Aabb)>,
    asset_store: Res<AssetStore>,
) {
    for event in events.read() {
        let Ok(background_hadle) = cells.get(event.entity) else {
            continue;
        };
        let Ok((material_handle, mut aabb)) = background.get_mut(background_hadle.0) else {
            continue;
        };
        let Some(material) = materials_assets.get_mut(material_handle) else {
            continue;
        };

        material.color = event.style.color;
        material.texture = event.style.texture.and_then(|id| asset_store.get(&id));
        material.size = event.style.size;
        material.skew = event.style.skew;
        material.rounding = event.style.rounding.into();

        let (min_skew, max_skew) = if event.style.skew > 0.0 {
            (0.0, event.style.skew)
        } else {
            (event.style.skew, 0.0)
        };
        *aabb = Aabb::from_min_max(
            vec3(min_skew, -event.style.size.y, 0.0),
            vec3(event.style.size.x + max_skew, 0.0, 0.0),
        );
    }
}
