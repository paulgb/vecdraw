use crate::layer::{DrawContext, DrawState, Drawable, Layer};

use crate::color::Color;
use crate::gpu_data::{GpuBuffer, GpuSerializable};
use std::borrow::Cow;
use wgpu::{
    BlendComponent, BlendState, RenderPipeline, ShaderModuleDescriptor, ShaderSource,
    VertexBufferLayout,
};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Zeroable, bytemuck::Pod)]
pub struct Line {
    pub start: [f32; 2],
    pub end: [f32; 2],
    pub color: Color,
    pub width: f32,
}

impl GpuSerializable for Line {
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
                    format: wgpu::VertexFormat::Unorm8x4,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
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
    pub instance_buffer: GpuBuffer<Line>,
}

impl Drawable for LinesLayerDrawable {
    fn draw<'a>(&'a self, draw_state: &DrawState<'a>) {
        let mut render_pass = draw_state.render_pass.borrow_mut();
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, draw_state.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.instance_buffer.all());
        render_pass.draw(0..6, 0..self.instance_buffer.len());
    }
}

impl Layer for LinesLayer {
    type D = LinesLayerDrawable;

    fn init_drawable(&self, draw_context: &DrawContext) -> LinesLayerDrawable {
        let DrawContext {
            device,
            sc_desc,
            transform_layout,
        } = *draw_context;
        let instance_buffer = GpuBuffer::new(&self.data, device);

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[transform_layout],
                push_constant_ranges: &[],
            });

        let shader_module = device.create_shader_module(&ShaderModuleDescriptor {
            label: None,
            source: ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
            flags: Default::default(),
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader_module,
                entry_point: "vs_main",
                buffers: &[Line::buffer_layout()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader_module,
                entry_point: "fs_main",
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

        LinesLayerDrawable {
            render_pipeline,
            instance_buffer,
        }
    }
}
