use bevy::{
    asset::AssetServer,
    ecs::system::Res,
    math::vec3,
    prelude::{Assets, Component, Entity, EventReader, Handle, Query, ResMut},
    render::{primitives::Aabb, view::RenderLayers},
};

use crate::{
    asset_path_store::{AssetPathProvider, AssetPathStore},
    cell_material::CellMaterial,
};

use super::SetStyle;

#[derive(Component)]
pub struct Background(pub Entity);

pub fn update_style(
    mut events: EventReader<SetStyle>,
    mut materials_assets: ResMut<Assets<CellMaterial>>,
    cells: Query<&Background>,
    mut background: Query<(&Handle<CellMaterial>, &mut Aabb, &mut RenderLayers)>,
    asset_server: Res<AssetServer>,
    asset_path_store: ResMut<AssetPathStore>,
) {
    for event in events.read() {
        let Ok(background_hadle) = cells.get(event.entity) else {
            continue;
        };
        let Ok((material_handle, mut aabb, mut render_layers)) =
            background.get_mut(background_hadle.0)
        else {
            continue;
        };
        let Some(material) = materials_assets.get_mut(material_handle) else {
            continue;
        };

        material.color = event.style.color;
        material.texture = event
            .style
            .texture
            .as_ref()
            .and_then(|id| asset_path_store.get(id))
            .and_then(|path| Some(asset_server.load(path)));
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

        *render_layers = RenderLayers::layer(event.style.render_layer);
    }
}
