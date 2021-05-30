use crate::layer::{DrawState, Drawable, Layer};

use wgpu::{BindGroupLayout, BlendComponent, BlendState, Device, RenderPipeline, SwapChainDescriptor, VertexBufferLayout};
use crate::gpu_data::{GPUSerializable, GPUBuffer};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Zeroable, bytemuck::Pod)]
pub struct Line {
    pub start: [f32; 2],
    pub end: [f32; 2],
    pub color: [f32; 4],
    pub width: f32,
}

impl GPUSerializable for Line {
    fn gpu_serialize(data: &[Self]) -> &[u8] {
        bytemuck::cast_slice(data)
    }

    fn buffer_layout<'a>() -> VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Line>() as wgpu::BufferAddress,
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
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32,
                },
            ],
        }
    }
}

pub struct LinesLayer {
    data: Vec<Line>,
}

impl LinesLayer {
    pub fn new(data: Vec<Line>) -> Self {
        LinesLayer { data }
    }
}

pub struct LinesLayerDrawable {
    render_pipeline: RenderPipeline,
    instance_buffer: GPUBuffer<Line>,
}

impl Drawable for LinesLayerDrawable {
    fn draw<'a>(&'a self, draw_state: &DrawState<'a>) {
        let mut render_pass = draw_state.render_pass.borrow_mut();
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass
            .set_bind_group(0, draw_state.bind_group, &[]);
        render_pass
            .set_vertex_buffer(0, self.instance_buffer.all());
        render_pass.draw(0..6, 0..self.instance_buffer.len());
    }
}

impl Layer for LinesLayer {
    fn init_drawable(
        &self,
        device: &Device,
        sc_desc: &SwapChainDescriptor,
        transform_layout: &BindGroupLayout,
    ) -> Box<dyn Drawable> {
        let instance_buffer = GPUBuffer::new(&self.data, device);

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
                buffers: &[Line::buffer_layout()],
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

        Box::new(LinesLayerDrawable {
            render_pipeline,
            instance_buffer,
        })
    }
}
