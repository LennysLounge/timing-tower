use std::collections::HashMap;

use bevy::{
    math::vec3,
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
};
use common::communication::StyleCommand;
use uuid::Uuid;

use crate::{
    asset_path_store::{AssetPathProvider, AssetPathStore},
    cell::{CellStyle, CreateCell, CreateClipArea, SetStyle},
};

#[derive(Default)]
pub struct CellManager {
    cells: HashMap<Uuid, Entity>,
    clip_areas: HashMap<Uuid, (Entity, Handle<Image>, Entity)>,
}
impl CellManager {
    /// Apply style commands and update/create/remove the style of cells.
    #[allow(unused)]
    pub fn apply_commands(
        &mut self,
        style_commands: Vec<StyleCommand>,
        mut set_style: EventWriter<SetStyle>,
        mut commands: Commands,
        mut images: ResMut<Assets<Image>>,
        mut cameras: Query<&mut Transform, With<Camera>>,
        asset_server: Res<AssetServer>,
        asset_path_store: ResMut<AssetPathStore>,
    ) {
        for command in style_commands.into_iter() {
            match command {
                StyleCommand::Style { id, style } => {
                    let cell_id = self
                        .cells
                        .entry(id)
                        .or_insert_with(|| commands.spawn_empty().add(CreateCell).id());

                    set_style.send(SetStyle {
                        entity: *cell_id,
                        style: CellStyle {
                            text: style.text.clone(),
                            text_color: style.text_color,
                            text_size: style.text_size,
                            text_alignment: style.text_alignment,
                            text_position: style.text_position,
                            color: style.color,
                            texture: style
                                .texture
                                .as_ref()
                                .and_then(|id| asset_path_store.get(id))
                                .and_then(|path| Some(asset_server.load(path))),
                            pos: style.pos,
                            size: style.size,
                            skew: style.skew,
                            visible: style.visible,
                            rounding: style.rounding,
                            render_layer: style.render_layer,
                        },
                    });
                }
                StyleCommand::ClipArea { id, style } => {
                    let (clip_area_id, texture, camera_id) =
                        self.clip_areas.entry(id).or_insert_with(|| {
                            let image = Image {
                                texture_descriptor: TextureDescriptor {
                                    label: None,
                                    size: Extent3d::default(),
                                    mip_level_count: 1,
                                    sample_count: 1,
                                    dimension: TextureDimension::D2,
                                    format: TextureFormat::Bgra8UnormSrgb,
                                    usage: TextureUsages::TEXTURE_BINDING
                                        | TextureUsages::COPY_DST
                                        | TextureUsages::RENDER_ATTACHMENT,
                                    view_formats: &[],
                                },
                                ..Default::default()
                            };
                            let image_handle = images.add(image);
                            (
                                commands.spawn_empty().add(CreateClipArea).id(),
                                image_handle.clone(),
                                commands
                                    .spawn(Camera2dBundle {
                                        camera: Camera {
                                            order: -1,
                                            target: RenderTarget::Image(image_handle),
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    })
                                    .insert(RenderLayers::layer(style.render_layer))
                                    .id(),
                            )
                        });

                    if let Some(image) = images.get_mut(texture.id()) {
                        image.resize(Extent3d {
                            width: style.size.x as u32,
                            height: style.size.y as u32,
                            ..Default::default()
                        });
                    }
                    if let Ok(mut camera) = cameras.get_mut(*camera_id) {
                        camera.translation =
                            style.pos + vec3(style.size.x / 2.0, -style.size.y / 2.0, 0.0);
                    }

                    set_style.send(SetStyle {
                        entity: *clip_area_id,
                        style: CellStyle {
                            pos: style.pos,
                            size: style.size,
                            skew: style.skew,
                            rounding: style.rounding,
                            color: Color::WHITE,
                            visible: true,
                            texture: Some(texture.clone()),
                            ..Default::default()
                        },
                    });
                }
                StyleCommand::Remove { id } => {
                    if let Some(cell_id) = self.cells.remove(&id) {
                        commands.entity(cell_id).despawn_recursive();
                    }

                    if let Some((cell_id, _, camera_id)) = self.clip_areas.remove(&id) {
                        commands.entity(cell_id).despawn_recursive();
                        commands.entity(camera_id).despawn();
                    }
                }
            }
        }
    }
}
