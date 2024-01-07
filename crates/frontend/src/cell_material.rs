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
        mesh::{GpuBufferInfo, MeshVertexBufferLayout},
        render_asset::{prepare_assets, RenderAssets},
        render_phase::{
            AddRenderCommand, DrawFunctions, PhaseItem, RenderCommand, RenderCommandResult,
            RenderPhase, SetItemPipeline, TrackedRenderPass,
        },
        render_resource::*,
        renderer::RenderDevice,
        texture::FallbackImage,
        view::{ExtractedView, RenderLayers, VisibleEntities},
        Extract, Render, RenderApp, RenderSet,
    },
    sprite::{
        Mesh2dHandle, Mesh2dPipeline, Mesh2dPipelineKey, RenderMesh2dInstances,
        SetMesh2dViewBindGroup,
    },
    utils::FloatOrd,
};
use bytemuck::{Pod, Zeroable};
use uuid::uuid;

const VERT_SHADER_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(uuid!("8f2e85d4-c560-410c-9159-c37a95e865e5").as_u128());
const FRAG_SHADER_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(uuid!("eb34f151-aa39-4148-8e01-7c801b4b8566").as_u128());
const INSTANCE_SHADER_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(uuid!("a42b398a-0d38-4f78-aea0-c737b10ba73f").as_u128());

pub struct CellMaterialPlugin;
impl Plugin for CellMaterialPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            FRAG_SHADER_HANDLE,
            "../shaders/cell_frag.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            VERT_SHADER_HANDLE,
            "../shaders/cell_vert.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            INSTANCE_SHADER_HANDLE,
            "../shaders/instanced_cell_material.wgsl",
            Shader::from_wgsl
        );
        app.add_plugins(ExtractComponentPlugin::<CellMaterial>::default());
        app.sub_app_mut(RenderApp)
            .add_render_command::<Transparent2d, DrawCellMaterial>()
            .init_resource::<SpecializedMeshPipelines<CellMaterialPipline>>()
            .add_systems(ExtractSchedule, extract_mesh_2d_handle)
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
    pub skew: f32,
    pub rounding: [f32; 4],
    pub color: Color,
    pub texture: Option<Handle<Image>>,
}
#[derive(Component)]
pub struct ExtractedCellMaterial {
    material: CellMaterial,
    mesh: Mesh2dHandle,
    transform: Vec3,
    render_layer: RenderLayers,
}

impl ExtractComponent for CellMaterial {
    type Query = (
        &'static CellMaterial,
        &'static Mesh2dHandle,
        &'static GlobalTransform,
        &'static RenderLayers,
        &'static InheritedVisibility,
    );
    type Filter = ();
    type Out = ExtractedCellMaterial;

    fn extract_component(
        (material, mesh, transform, layers, visibility): QueryItem<'_, Self::Query>,
    ) -> Option<Self::Out> {
        visibility.get().then_some(ExtractedCellMaterial {
            material: material.clone(),
            mesh: mesh.clone(),
            transform: transform.translation(),
            render_layer: *layers,
        })
    }
}

fn extract_mesh_2d_handle(mut commands: Commands, query: Extract<Query<(Entity, &Mesh2dHandle)>>) {
    let mut extracted = Vec::new();
    for (e, m) in query.iter() {
        extracted.push((e, m.clone()));
    }
    commands.insert_or_spawn_batch(extracted);
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
    skew: f32,
    rounding: [f32; 4],
    color: [f32; 4],
}

/// The extracted and grouped cell material data
#[derive(Component)]
struct GroupedCellMaterial {
    uniform: UniformData,
    per_instance: Vec<InstanceData>,
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
                extracted.mesh.0.hash(&mut hasher);
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
                        per_instance: group
                            .iter()
                            .map(|(_, extracted)| InstanceData {
                                position: extracted.transform,
                                size: extracted.material.size,
                                skew: extracted.material.skew,
                                rounding: extracted.material.rounding,
                                color: extracted.material.color.as_linear_rgba_f32(),
                            })
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
    //println!("start uniform buffer");
    for (entity, material) in query.iter() {
        //println!("\tentity:{entity:?}");
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
    mut pipelines: ResMut<SpecializedMeshPipelines<CellMaterialPipline>>,
    pipeline_cache: Res<PipelineCache>,
    meshes: Res<RenderAssets<Mesh>>,
    render_mesh_instances: Res<RenderMesh2dInstances>,
    material_meshes: Query<Entity, (With<UniformBuffer>, With<GroupedCellMaterial>)>,
    mut views: Query<(
        &ExtractedView,
        &VisibleEntities,
        &mut RenderPhase<Transparent2d>,
    )>,
) {
    //println!("Start queue");
    let draw_custom = transparent_2d_draw_functions
        .read()
        .id::<DrawCellMaterial>();

    let msaa_key = Mesh2dPipelineKey::from_msaa_samples(msaa.samples());

    for (view, visible_entities, mut transparent_phase) in &mut views {
        //println!("we have a view");
        let view_key = msaa_key | Mesh2dPipelineKey::from_hdr(view.hdr);
        for visible_entity in &visible_entities.entities {
            //println!("there is an entity");
            let Ok(_) = material_meshes.get(*visible_entity) else {
                continue;
            };
            let Some(mesh_instance) = render_mesh_instances.get(visible_entity) else {
                continue;
            };
            let Some(mesh) = meshes.get(mesh_instance.mesh_asset_id) else {
                continue;
            };
            let key =
                view_key | Mesh2dPipelineKey::from_primitive_topology(mesh.primitive_topology);
            let pipeline = pipelines
                .specialize(&pipeline_cache, &custom_pipeline, key, &mesh.layout)
                .unwrap();
            //println!("Add phase item for entity: {:?}", entity);
            transparent_phase.add(Transparent2d {
                entity: *visible_entity,
                pipeline,
                draw_function: draw_custom,
                batch_range: 0..1,
                dynamic_offset: None,
                sort_key: FloatOrd(mesh_instance.transforms.transform.translation.z),
            });
        }
    }
    //println!("end queue");
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

impl SpecializedMeshPipeline for CellMaterialPipline {
    type Key = Mesh2dPipelineKey;

    fn specialize(
        &self,
        key: Self::Key,
        layout: &MeshVertexBufferLayout,
    ) -> Result<RenderPipelineDescriptor, SpecializedMeshPipelineError> {
        let mut descriptor = self.mesh2d_pipeline.specialize(key, layout)?;

        // We are only using the view binding and uniform data here.
        descriptor.layout = vec![
            self.mesh2d_pipeline.view_layout.clone(),
            self.uniform_data_layout.clone(),
        ];

        descriptor.vertex.shader = INSTANCE_SHADER_HANDLE.clone();
        descriptor.vertex.buffers.push(VertexBufferLayout {
            array_stride: std::mem::size_of::<InstanceData>() as u64,
            step_mode: VertexStepMode::Instance,
            attributes: vec![
                VertexAttribute {
                    format: VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 3, // shader locations 0-2 are taken up by Position, Normal and UV attributes
                },
                VertexAttribute {
                    format: VertexFormat::Float32x2,
                    offset: 12,
                    shader_location: 4, // shader locations 0-2 are taken up by Position, Normal and UV attributes
                },
                VertexAttribute {
                    format: VertexFormat::Float32,
                    offset: 20,
                    shader_location: 5, // shader locations 0-2 are taken up by Position, Normal and UV attributes
                },
                VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: 24,
                    shader_location: 6,
                },
                VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: 40,
                    shader_location: 7,
                },
            ],
        });
        descriptor.fragment.as_mut().unwrap().shader = INSTANCE_SHADER_HANDLE.clone();
        Ok(descriptor)
    }
}

type DrawCellMaterial = (
    SetItemPipeline,
    SetMesh2dViewBindGroup<0>,
    DrawMesh2dInstanced,
);

pub struct DrawMesh2dInstanced;

impl<P: PhaseItem> RenderCommand<P> for DrawMesh2dInstanced {
    type Param = (SRes<RenderAssets<Mesh>>, SRes<RenderMesh2dInstances>);
    type ViewWorldQuery = ();
    type ItemWorldQuery = (Read<UniformBuffer>, Read<InstanceBuffer>);

    #[inline]
    fn render<'w>(
        item: &P,
        _view: (),
        (_uniform_buffer, instance_buffer): (&'w UniformBuffer, &'w InstanceBuffer),
        (meshes, render_mesh_instances): SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        //println!("do draw call for entity: {:?}", item.entity());
        let Some(mesh_instance) = render_mesh_instances.get(&item.entity()) else {
            return RenderCommandResult::Failure;
        };
        let gpu_mesh = match meshes.into_inner().get(mesh_instance.mesh_asset_id) {
            Some(gpu_mesh) => gpu_mesh,
            None => return RenderCommandResult::Failure,
        };
        pass.set_bind_group(1, &_uniform_buffer.prepared.bind_group, &[]);
        pass.set_vertex_buffer(0, gpu_mesh.vertex_buffer.slice(..));
        pass.set_vertex_buffer(1, instance_buffer.buffer.slice(..));

        match &gpu_mesh.buffer_info {
            GpuBufferInfo::Indexed {
                buffer,
                index_format,
                count,
            } => {
                pass.set_index_buffer(buffer.slice(..), 0, *index_format);
                pass.draw_indexed(0..*count, 0, 0..instance_buffer.length as u32);
            }
            GpuBufferInfo::NonIndexed => {
                pass.draw(0..gpu_mesh.vertex_count, 0..instance_buffer.length as u32);
            }
        }
        RenderCommandResult::Success
    }
}
