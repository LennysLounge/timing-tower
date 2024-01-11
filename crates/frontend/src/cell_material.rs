//! A material that renders cells as instanced.

use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use bevy::{
    asset::load_internal_asset,
    core_pipeline::core_2d::Transparent2d,
    ecs::{
        query::QueryItem,
        system::{lifetimeless::*, SystemParamItem},
    },
    prelude::*,
    render::{
        extract_component::{ExtractComponent, ExtractComponentPlugin},
        render_asset::{prepare_assets, RenderAssets},
        render_phase::{
            AddRenderCommand, DrawFunctions, PhaseItem, RenderCommand, RenderCommandResult,
            RenderPhase, SetItemPipeline, TrackedRenderPass,
        },
        render_resource::*,
        renderer::RenderDevice,
        texture::{BevyDefault, FallbackImage},
        view::{ExtractedView, RenderLayers, VisibleEntities},
        Render, RenderApp, RenderSet,
    },
    sprite::{Mesh2dPipeline, Mesh2dPipelineKey, SetMesh2dViewBindGroup},
    utils::FloatOrd,
};
use bytemuck::{Pod, Zeroable};
use uuid::uuid;

const INSTANCE_SHADER_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(uuid!("a42b398a-0d38-4f78-aea0-c737b10ba73f").as_u128());

pub struct CellMaterialPlugin;
impl Plugin for CellMaterialPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            INSTANCE_SHADER_HANDLE,
            "../shaders/instanced_cell_material.wgsl",
            Shader::from_wgsl
        );
        app.add_plugins(ExtractComponentPlugin::<CellMaterial>::default());
        app.sub_app_mut(RenderApp)
            .add_render_command::<Transparent2d, DrawCellMaterial>()
            .init_resource::<SpecializedRenderPipelines<CellMaterialPipline>>()
            .add_systems(
                Render,
                (
                    (group_instance_data, apply_deferred, prepare_uniform_buffers)
                        .chain()
                        .after(prepare_assets::<Image>),
                    queue_custom.in_set(RenderSet::QueueMeshes),
                    prepare_instance_buffers.in_set(RenderSet::PrepareResources),
                ),
            );
    }

    fn finish(&self, app: &mut App) {
        app.sub_app_mut(RenderApp)
            .init_resource::<CellMaterialPipline>();
    }
}

/// A material to render a cell.
/// Cells are rendered as instances.
#[derive(Component, Default, Clone)]
pub struct CellMaterial {
    pub size: Vec2,
    pub corner_offsets: [Vec2; 4],
    pub rounding: [f32; 4],
    pub color: Color,
    pub texture: Option<Handle<Image>>,
}
#[derive(Component)]
pub struct ExtractedCellMaterial {
    material: CellMaterial,
    transform: Vec3,
    render_layer: RenderLayers,
}

impl ExtractComponent for CellMaterial {
    type Query = (
        &'static CellMaterial,
        &'static GlobalTransform,
        &'static RenderLayers,
        &'static InheritedVisibility,
    );
    type Filter = ();
    type Out = ExtractedCellMaterial;

    fn extract_component(
        (material, transform, layers, visibility): QueryItem<'_, Self::Query>,
    ) -> Option<Self::Out> {
        visibility.get().then_some(ExtractedCellMaterial {
            material: material.clone(),
            transform: transform.translation(),
            render_layer: *layers,
        })
    }
}
impl ExtractedCellMaterial {
    fn to_instance_data(&self) -> InstanceData {
        InstanceData {
            position: self.transform,
            size: self.material.size,
            corner_offset_x: [
                self.material.corner_offsets[0].x,
                self.material.corner_offsets[1].x,
                self.material.corner_offsets[2].x,
                self.material.corner_offsets[3].x,
            ],
            corner_offset_y: [
                self.material.corner_offsets[0].y,
                self.material.corner_offsets[1].y,
                self.material.corner_offsets[2].y,
                self.material.corner_offsets[3].y,
            ],
            rounding: self.material.rounding,
            color: self.material.color.as_linear_rgba_f32(),
        }
    }
}

/// The data that stays the same for each group of cells.
#[derive(AsBindGroup)]
struct UniformData {
    #[texture(0)]
    #[sampler(1)]
    texture: Option<Handle<Image>>,
}

/// The data that is associated with each instance of a cell.
#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
struct InstanceData {
    position: Vec3,
    size: Vec2,
    corner_offset_x: [f32; 4],
    corner_offset_y: [f32; 4],
    rounding: [f32; 4],
    color: [f32; 4],
}

/// The extracted and grouped cell material data
#[derive(Component)]
struct GroupedCellMaterial {
    uniform: UniformData,
    per_instance: Vec<InstanceData>,
    z_pos: f32,
}

fn group_instance_data(mut commands: Commands, query: Query<(Entity, &ExtractedCellMaterial)>) {
    // group by render layer
    // Cells can only belong to one render layer at the moment.
    // This restriction is because the same host entity might get asigned multiple render groups.
    // Once we no longe rely on the host entity from the mesh we can create as many entities
    // as are needed to render the groups.
    let mut layers: [Vec<(Entity, &ExtractedCellMaterial)>; 32] = Default::default();
    query.iter().for_each(|x| {
        for layer_index in 0..32 {
            let layer = RenderLayers::layer(layer_index);
            if x.1.render_layer.intersects(&layer) {
                layers[layer_index as usize].push(x);
                return;
            }
        }
    });

    for layer in layers {
        let mut sorted_entries = layer
            .into_iter()
            .map(|(entity, extracted)| {
                let mut hasher = DefaultHasher::new();
                extracted.material.texture.hash(&mut hasher);
                (entity, extracted, hasher.finish())
            })
            .collect::<Vec<_>>();
        sorted_entries.sort_by(|(_, extracted_a, hash_a), (_, extracted_b, hash_b)| {
            extracted_a
                .transform
                .z
                .partial_cmp(&extracted_b.transform.z)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(hash_a.cmp(&hash_b))
        });

        let (mut groups, _, acc) = sorted_entries.into_iter().fold(
            (Vec::new(), 0u64, Vec::new()),
            |(mut groups, mut current_hash, mut acc), (entity, extracted, extracted_hash)| {
                if current_hash == extracted_hash {
                    acc.push((entity, extracted));
                } else {
                    groups.push(acc);
                    current_hash = extracted_hash;
                    acc = vec![(entity, extracted)]
                }
                (groups, current_hash, acc)
            },
        );
        groups.push(acc);

        let entities = groups
            .iter_mut()
            .filter_map(|group| {
                if group.is_empty() {
                    return None;
                }
                let (host_entity, extracted) = group.first().unwrap();
                Some((
                    *host_entity,
                    GroupedCellMaterial {
                        uniform: UniformData {
                            texture: extracted.material.texture.clone(),
                        },
                        z_pos: extracted.transform.z,
                        per_instance: group
                            .iter()
                            .map(|(_, extracted)| extracted.to_instance_data())
                            .collect(),
                    },
                ))
            })
            .collect::<Vec<_>>();
        commands.insert_or_spawn_batch(entities);
    }
}

#[derive(Component)]
pub struct UniformBuffer {
    prepared: PreparedBindGroup<()>,
}

fn prepare_uniform_buffers(
    mut commands: Commands,
    query: Query<(Entity, &GroupedCellMaterial)>,
    pipeline: Res<CellMaterialPipline>,
    render_device: Res<RenderDevice>,
    images: Res<RenderAssets<Image>>,
    fallback_image: Res<FallbackImage>,
) {
    for (entity, material) in query.iter() {
        // Should this be cached between frames? I dont know.
        match material.uniform.as_bind_group(
            &pipeline.uniform_data_layout,
            &render_device,
            &images,
            &fallback_image,
        ) {
            Ok(bind_group) => {
                commands.entity(entity).insert(UniformBuffer {
                    prepared: bind_group,
                });
            }
            Err(_e) => {
                //println!("Failed to create uniform buffer: {e:?}");
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn queue_custom(
    transparent_2d_draw_functions: Res<DrawFunctions<Transparent2d>>,
    custom_pipeline: Res<CellMaterialPipline>,
    msaa: Res<Msaa>,
    mut pipelines: ResMut<SpecializedRenderPipelines<CellMaterialPipline>>,
    pipeline_cache: Res<PipelineCache>,
    material_meshes: Query<(Entity, &GroupedCellMaterial), With<UniformBuffer>>,
    mut views: Query<(
        &ExtractedView,
        &VisibleEntities,
        &mut RenderPhase<Transparent2d>,
    )>,
) {
    let draw_custom = transparent_2d_draw_functions
        .read()
        .id::<DrawCellMaterial>();

    let msaa_key = Mesh2dPipelineKey::from_msaa_samples(msaa.samples());

    for (view, visible_entities, mut transparent_phase) in &mut views {
        let key = msaa_key | Mesh2dPipelineKey::from_hdr(view.hdr);
        for visible_entity in &visible_entities.entities {
            let Ok((_entity, grouped_cell_material)) = material_meshes.get(*visible_entity) else {
                continue;
            };
            let pipeline = pipelines.specialize(&pipeline_cache, &custom_pipeline, key);
            transparent_phase.add(Transparent2d {
                entity: *visible_entity,
                pipeline,
                draw_function: draw_custom,
                batch_range: 0..1,
                dynamic_offset: None,
                sort_key: FloatOrd(grouped_cell_material.z_pos),
            });
        }
    }
}

#[derive(Component)]
pub struct InstanceBuffer {
    buffer: Buffer,
    length: usize,
}

fn prepare_instance_buffers(
    mut commands: Commands,
    query: Query<(Entity, &GroupedCellMaterial)>,
    render_device: Res<RenderDevice>,
) {
    //println!("start prepare");
    for (entity, material) in &query {
        //println!("prepare instance buffer for {entity:?}");
        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("instance data buffer"),
            contents: bytemuck::cast_slice(material.per_instance.as_slice()),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });
        //println!("instance_buffer: {:?}", buffer.id());
        commands.entity(entity).insert(InstanceBuffer {
            buffer,
            length: material.per_instance.len(),
        });
    }
    //println!("end prepare");
}

#[derive(Resource)]
pub struct CellMaterialPipline {
    mesh2d_pipeline: Mesh2dPipeline,
    uniform_data_layout: BindGroupLayout,
}

impl FromWorld for CellMaterialPipline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let uniform_data_layout = UniformData::bind_group_layout(render_device);

        let mesh2d_pipeline = world.resource::<Mesh2dPipeline>().clone();

        CellMaterialPipline {
            mesh2d_pipeline,
            uniform_data_layout,
        }
    }
}

impl SpecializedRenderPipeline for CellMaterialPipline {
    type Key = Mesh2dPipelineKey;

    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor {
        let vertex_buffer_layout = VertexBufferLayout {
            array_stride: std::mem::size_of::<InstanceData>() as u64,
            step_mode: VertexStepMode::Instance,
            attributes: vec![
                // model_pos
                VertexAttribute {
                    format: VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 0,
                },
                //size
                VertexAttribute {
                    format: VertexFormat::Float32x2,
                    offset: 12,
                    shader_location: 1,
                },
                // corner_offset_x
                VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: 20,
                    shader_location: 2,
                },
                // corner_offset_y
                VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: 36,
                    shader_location: 3,
                },
                // rounding
                VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: 52,
                    shader_location: 4,
                },
                // color
                VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: 68,
                    shader_location: 5,
                },
            ],
        };

        let mut push_constant_ranges = Vec::with_capacity(1);
        if cfg!(all(feature = "webgl", target_arch = "wasm32")) {
            push_constant_ranges.push(PushConstantRange {
                stages: ShaderStages::VERTEX,
                range: 0..4,
            });
        }

        RenderPipelineDescriptor {
            vertex: VertexState {
                shader: INSTANCE_SHADER_HANDLE.clone(),
                entry_point: "vertex".into(),
                shader_defs: Vec::new(),
                buffers: vec![vertex_buffer_layout],
            },
            fragment: Some(FragmentState {
                shader: INSTANCE_SHADER_HANDLE.clone(),
                shader_defs: Vec::new(),
                entry_point: "fragment".into(),
                targets: vec![Some(ColorTargetState {
                    format: TextureFormat::bevy_default(),
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            layout: vec![
                self.mesh2d_pipeline.view_layout.clone(),
                self.uniform_data_layout.clone(),
            ],
            push_constant_ranges,
            primitive: PrimitiveState {
                front_face: FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false,
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
            },
            depth_stencil: None,
            multisample: MultisampleState {
                count: key.msaa_samples(),
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            label: Some("cell_pipeline".into()),
        }
    }
}

type DrawCellMaterial = (
    SetItemPipeline,
    SetMesh2dViewBindGroup<0>,
    DrawMesh2dInstanced,
);

pub struct DrawMesh2dInstanced;

impl<P: PhaseItem> RenderCommand<P> for DrawMesh2dInstanced {
    type Param = ();
    type ViewWorldQuery = ();
    type ItemWorldQuery = (Read<UniformBuffer>, Read<InstanceBuffer>);

    #[inline]
    fn render<'w>(
        _item: &P,
        _view: (),
        (uniform_buffer, instance_buffer): (&'w UniformBuffer, &'w InstanceBuffer),
        _: SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        //println!("do draw call for entity: {:?}", item.entity());

        pass.set_bind_group(1, &uniform_buffer.prepared.bind_group, &[]);
        pass.set_vertex_buffer(0, instance_buffer.buffer.slice(..));
        pass.draw(0..6, 0..instance_buffer.length as u32);

        RenderCommandResult::Success
    }
}
