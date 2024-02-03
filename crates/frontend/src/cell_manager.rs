use std::{cmp::max, collections::HashMap};

use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
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
    cell::{CellStyle, CreateCell, CreateClipArea, SetStyle},
    AssetPathProvider,
};

#[derive(Default)]
pub struct CellManager {
    cells: HashMap<Uuid, Entity>,
    clip_areas: HashMap<Uuid, ClipArea>,
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
        mut cameras: Query<(&mut Transform, &mut RenderLayers), With<Camera>>,
        asset_server: Res<AssetServer>,
        asset_path_store: &impl AssetPathProvider,
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
                            font: style
                                .font
                                .as_ref()
                                .and_then(|id| asset_path_store.get(id))
                                .and_then(|path| Some(asset_server.load(path))),
                            color: style.color,
                            texture: style
                                .texture
                                .as_ref()
                                .and_then(|id| asset_path_store.get(id))
                                .and_then(|path| Some(asset_server.load(path))),
                            pos: style.pos,
                            size: style.size,
                            corner_offsets: style.corner_offsets,
                            visible: style.visible,
                            rounding: style.rounding,
                            render_layer: style.render_layer,
                        },
                    });
                }
                StyleCommand::ClipArea { id, style } => {
                    let ClipArea {
                        cell,
                        texture,
                        camera,
                    } = self
                        .clip_areas
                        .entry(id)
                        .or_insert_with(|| ClipArea::new(&mut commands, &mut images));

                    if let Some(image) = images.get_mut(texture.id()) {
                        image.resize(Extent3d {
                            width: max(style.size.x as u32, 1),
                            height: max(style.size.y as u32, 1),
                            ..Default::default()
                        });
                    }
                    if let Ok((mut camera, mut render_layers)) = cameras.get_mut(*camera) {
                        camera.translation =
                            style.pos + vec3(style.size.x / 2.0, -style.size.y / 2.0, 0.0);
                        if style.render_layer > 0 {
                            *render_layers = RenderLayers::layer(style.render_layer);
                        } else {
                            *render_layers = RenderLayers::none();
                        }
                    }

                    set_style.send(SetStyle {
                        entity: *cell,
                        style: CellStyle {
                            pos: style.pos,
                            size: style.size,
                            corner_offsets: style.corner_offsets,
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

                    if let Some(ClipArea {
                        cell,
                        texture,
                        camera,
                    }) = self.clip_areas.remove(&id)
                    {
                        commands.entity(cell).despawn_recursive();
                        commands.entity(camera).despawn();
                        images.remove(texture);
                    }
                }
            }
        }
    }
}

struct ClipArea {
    cell: Entity,
    texture: Handle<Image>,
    camera: Entity,
}

impl ClipArea {
    fn new(commands: &mut Commands, images: &mut Assets<Image>) -> Self {
        let clip_area = commands.spawn_empty().add(CreateClipArea).id();
        let texture = images.add(Image {
            texture_descriptor: TextureDescriptor {
                label: None,
                size: Extent3d::default(),
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: TextureFormat::Rgba8UnormSrgb,
                usage: TextureUsages::TEXTURE_BINDING
                    | TextureUsages::COPY_DST
                    | TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            },
            ..Default::default()
        });
        let camera = commands
            .spawn(Camera2dBundle {
                camera: Camera {
                    order: -1,
                    target: RenderTarget::Image(texture.clone()),
                    ..Default::default()
                },
                camera_2d: Camera2d {
                    clear_color: ClearColorConfig::Custom(Color::rgba_u8(0, 0, 0, 0)),
                },
                ..Default::default()
            })
            .insert(RenderLayers::layer(0))
            .id();
        Self {
            cell: clip_area,
            texture,
            camera,
        }
    }
}
