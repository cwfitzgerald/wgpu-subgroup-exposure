use std::ptr::null;

use wgpu::{
    include_spirv, util::DeviceExt, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, BufferBindingType, CommandEncoderDescriptor,
    CompareFunction, DepthBiasState, DepthStencilState, Extent3d, FragmentState, LoadOp,
    MultisampleState, Operations, PipelineLayoutDescriptor, PrimitiveState,
    RenderPassDepthStencilAttachmentDescriptor, RenderPassDescriptor, RenderPipelineDescriptor,
    ShaderStage, StencilState, TextureDescriptor, TextureDimension, TextureFormat, TextureUsage,
    TextureViewDescriptor, VertexState,
};

fn main() {
    let instance = wgpu::Instance::new(wgpu::BackendBit::VULKAN);
    let adapter = instance
        .enumerate_adapters(wgpu::BackendBit::VULKAN)
        .nth(2)
        .unwrap();
    dbg!(adapter.get_info());
    let (device, queue) = pollster::block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            features: wgpu::Features::MAPPABLE_PRIMARY_BUFFERS,
            ..Default::default()
        },
        None,
    ))
    .unwrap();

    // let mut rd = renderdoc::RenderDoc::<renderdoc::V141>::new().unwrap();

    // rd.start_frame_capture(null(), null());

    let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: None,
        contents: &*vec![0_u8; 4 * 32],
        usage: wgpu::BufferUsage::COPY_DST
            | wgpu::BufferUsage::MAP_READ
            | wgpu::BufferUsage::STORAGE,
    });

    let subgroup_vert = device.create_shader_module(&include_spirv!("subgroup.vert.spv"));
    let subgroup_frag = device.create_shader_module(&include_spirv!("subgroup.frag.spv"));
    let fullscreen_vert = device.create_shader_module(&include_spirv!("fullscreen.vert.spv"));

    let bgl = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: None,
        entries: &[BindGroupLayoutEntry {
            binding: 0,
            count: None,
            ty: BindingType::Buffer {
                ty: BufferBindingType::Storage { read_only: false },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            visibility: ShaderStage::all(),
        }],
    });

    let bg = device.create_bind_group(&BindGroupDescriptor {
        label: None,
        layout: &bgl,
        entries: &[BindGroupEntry {
            binding: 0,
            resource: buffer.as_entire_binding(),
        }],
    });

    let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[&bgl],
        push_constant_ranges: &[],
    });

    let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: VertexState {
            module: &fullscreen_vert,
            entry_point: "main",
            buffers: &[],
        },
        primitive: PrimitiveState::default(),
        depth_stencil: Some(DepthStencilState {
            format: TextureFormat::Depth32Float,
            depth_write_enabled: false,
            depth_compare: CompareFunction::Always,
            stencil: StencilState::default(),
            bias: DepthBiasState::default(),
            clamp_depth: false,
        }),
        multisample: MultisampleState::default(),
        fragment: Some(FragmentState {
            module: &subgroup_frag,
            entry_point: "main",
            targets: &[],
        }),
    });

    let dummy_target = device.create_texture(&TextureDescriptor {
        label: None,
        size: Extent3d {
            width: 4,
            height: 4,
            depth: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D2,
        format: TextureFormat::Depth32Float,
        usage: TextureUsage::RENDER_ATTACHMENT,
    });

    let view = dummy_target.create_view(&TextureViewDescriptor::default());

    let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor::default());

    let mut rpass = encoder.begin_render_pass(&RenderPassDescriptor {
        label: None,
        color_attachments: &[],
        depth_stencil_attachment: Some(RenderPassDepthStencilAttachmentDescriptor {
            attachment: &view,
            depth_ops: Some(Operations {
                load: LoadOp::Load,
                store: false,
            }),
            stencil_ops: None,
        }),
    });
    rpass.set_pipeline(&pipeline);
    rpass.set_bind_group(0, &bg, &[]);
    rpass.draw(0..3, 0..2);
    drop(rpass);

    queue.submit(Some(encoder.finish()));

    // rd.end_frame_capture(null(), null());

    let _ = buffer.slice(..).map_async(wgpu::MapMode::Read);
    device.poll(wgpu::Maintain::Wait);
    let range = buffer.slice(..).get_mapped_range();
    let warp_count: Vec<f32> = bytemuck::cast_slice(&*range).to_vec();
    drop(range);
    buffer.unmap();

    dbg!(warp_count);
}
