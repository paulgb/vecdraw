use wgpu::util::DeviceExt;
use wgpu::{
    BindGroupLayout, BlendComponent, BlendState, Buffer, Device, RenderPipeline,
    SwapChainDescriptor,
};

use crate::layer::{DrawState, Drawable, Layer};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Zeroable, bytemuck::Pod)]
pub struct Circle {
    pub position: [f32; 2],
    pub color: [f32; 4],
    pub radius: f32,
}

#[derive(Debug)]
pub struct CirclesLayer {
    data: Vec<Circle>,
}

impl CirclesLayer {
    pub fn new(data: Vec<Circle>) -> Self {
        CirclesLayer { data }
    }
}

pub struct CirclesLayerDrawable {
    render_pipeline: RenderPipeline,
    instance_buffer: Buffer,
    num_circles: u32,
}

impl Drawable for CirclesLayerDrawable {
    fn draw<'a>(&'a self, draw_state: &DrawState<'a>) {
        let mut render_pass = draw_state.render_pass.borrow_mut();
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, draw_state.bind_group, &[]);
        render_pass
            .set_vertex_buffer(0, self.instance_buffer.slice(..));
        render_pass.draw(0..6, 0..self.num_circles);
    }
}

impl Layer for CirclesLayer {
    fn init_drawable(
        &self,
        device: &Device,
        sc_desc: &SwapChainDescriptor,
        transform_layout: &BindGroupLayout,
    ) -> Box<dyn Drawable> {
        let instance_buffer_desc = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Circle>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 6]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32,
                },
            ],
        };

        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance buffer"),
            contents: bytemuck::cast_slice(&self.data),
            usage: wgpu::BufferUsage::VERTEX,
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[transform_layout],
                push_constant_ranges: &[],
            });

        let vs_module = device.create_shader_module(&wgpu::include_spirv!("shader.vert.spv"));
        let fs_module = device.create_shader_module(&wgpu::include_spirv!("shader.frag.spv"));

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vs_module,
                entry_point: "main",
                buffers: &[instance_buffer_desc],
            },
            fragment: Some(wgpu::FragmentState {
                module: &fs_module,
                entry_point: "main",
                targets: &[wgpu::ColorTargetState {
                    format: sc_desc.format,
                    write_mask: wgpu::ColorWrite::ALL,
                    blend: Some(BlendState {
                        color: BlendComponent::OVER,
                        alpha: BlendComponent::REPLACE,
                    }),
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                //cull_mode: Some(wgpu::Face::Back),
                cull_mode: None,
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                clamp_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
        });

        Box::new(CirclesLayerDrawable {
            render_pipeline,
            instance_buffer,
            num_circles: self.data.len() as u32,
        })
    }
}
