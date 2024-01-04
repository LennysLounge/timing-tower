//! A material that renders cells as instanced.

use bevy::{
    core_pipeline::core_2d::Transparent2d,
    ecs::system::{lifetimeless::*, SystemParamItem},
    prelude::*,
    render::{
        mesh::{GpuBufferInfo, MeshVertexBufferLayout},
        render_asset::{prepare_assets, RenderAssets},
        render_phase::{
            AddRenderCommand, DrawFunctions, PhaseItem, RenderCommand, RenderCommandResult,
            RenderPhase, SetItemPipeline, TrackedRenderPass,
        },
        render_resource::*,
        renderer::RenderDevice,
        texture::FallbackImage,
        view::ExtractedView,
        Extract, Render, RenderApp, RenderSet,
    },
    sprite::{
        Mesh2dHandle, Mesh2dPipeline, Mesh2dPipelineKey, RenderMesh2dInstances,
        SetMesh2dViewBindGroup,
    },
    utils::{FloatOrd, HashMap},
};
use bytemuck::{Pod, Zeroable};

pub struct CellMaterialPlugin;
impl Plugin for CellMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.sub_app_mut(RenderApp)
            .add_render_command::<Transparent2d, DrawCellMaterial>()
            .init_resource::<SpecializedMeshPipelines<CellMaterialPipline>>()
            .add_systems(ExtractSchedule, extract_cell_material)
            .add_systems(
                Render,
                (
                    prepare_uniform_buffers
                        .in_set(RenderSet::PrepareAssets)
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
#[derive(Component, Default)]
pub struct CellMaterial {
    pub position: Vec3,
    pub scale: f32,
    pub color: Color,
    pub texture: Option<Handle<Image>>,
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
    scale: f32,
    color: [f32; 4],
}

/// The extracted and grouped cell material data
#[derive(Component)]
struct ExtractedCellMaterial {
    uniform: UniformData,
    per_instance: Vec<InstanceData>,
}

fn extract_cell_material(
    mut commands: Commands,
    query: Extract<Query<(Entity, &CellMaterial, &Mesh2dHandle, &GlobalTransform)>>,
) {
    // Group cells by their mesh and their background texture.
    type GroupKey<'a> = (&'a Handle<Mesh>, &'a Option<Handle<Image>>);
    let mut groups: HashMap<GroupKey, (Entity, ExtractedCellMaterial)> = HashMap::new();

    for (entity, material, mesh, transform) in query.iter() {
        let key = (&mesh.0, &material.texture);
        let (_entity, extracted) = groups.entry(key).or_insert_with(|| {
            (
                entity,
                ExtractedCellMaterial {
                    uniform: UniformData {
                        texture: material.texture.clone(),
                    },
                    per_instance: Vec::new(),
                },
            )
        });

        extracted.per_instance.push(InstanceData {
            position: material.position + transform.translation(),
            scale: material.scale,
            color: material.color.clone().into(),
        });
    }
    commands.insert_or_spawn_batch(groups.into_values().collect::<Vec<_>>());
}

#[derive(Component)]
pub struct UniformBuffer {
    prepared: PreparedBindGroup<()>,
}

fn prepare_uniform_buffers(
    mut commands: Commands,
    query: Query<(Entity, &ExtractedCellMaterial)>,
    pipeline: Res<CellMaterialPipline>,
    render_device: Res<RenderDevice>,
    images: Res<RenderAssets<Image>>,
    fallback_image: Res<FallbackImage>,
) {
    for (entity, material) in query.iter() {
        let x = material.uniform.as_bind_group(
            &pipeline.uniform_data_layout,
            &render_device,
            &images,
            &fallback_image,
        );
        let y = x.unwrap();
        commands
            .entity(entity)
            .insert(UniformBuffer { prepared: y });
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
    material_meshes: Query<Entity, (With<UniformBuffer>, With<ExtractedCellMaterial>)>,
    mut views: Query<(&ExtractedView, &mut RenderPhase<Transparent2d>)>,
) {
    //println!("Start queue");
    let draw_custom = transparent_2d_draw_functions
        .read()
        .id::<DrawCellMaterial>();

    let msaa_key = Mesh2dPipelineKey::from_msaa_samples(msaa.samples());

    for (view, mut transparent_phase) in &mut views {
        //println!("we have a view");
        let view_key = msaa_key | Mesh2dPipelineKey::from_hdr(view.hdr);
        for entity in &material_meshes {
            //println!("there is an entity");
            let Some(mesh_instance) = render_mesh_instances.get(&entity) else {
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
                entity,
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
    query: Query<(Entity, &ExtractedCellMaterial)>,
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
    shader: Handle<Shader>,
    mesh2d_pipeline: Mesh2dPipeline,
    uniform_data_layout: BindGroupLayout,
}

impl FromWorld for CellMaterialPipline {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let shader = asset_server.load("shaders/instancing2d.wgsl");

        let render_device = world.resource::<RenderDevice>();
        let uniform_data_layout = UniformData::bind_group_layout(render_device);

        let mesh2d_pipeline = world.resource::<Mesh2dPipeline>().clone();

        CellMaterialPipline {
            shader,
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

        descriptor.vertex.shader = self.shader.clone();
        descriptor.vertex.buffers.push(VertexBufferLayout {
            array_stride: std::mem::size_of::<InstanceData>() as u64,
            step_mode: VertexStepMode::Instance,
            attributes: vec![
                VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: 0,
                    shader_location: 3, // shader locations 0-2 are taken up by Position, Normal and UV attributes
                },
                VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: VertexFormat::Float32x4.size(),
                    shader_location: 4,
                },
            ],
        });
        descriptor.fragment.as_mut().unwrap().shader = self.shader.clone();
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
