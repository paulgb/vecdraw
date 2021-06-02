use wgpu::{
    BlendComponent, BlendState, RenderPipeline, ShaderModuleDescriptor, ShaderSource,
    VertexBufferLayout,
};

use crate::color::Color;
use crate::gpu_data::{GpuBuffer, GpuSerializable};
use crate::layer::{DrawContext, DrawState, Drawable, Layer};
use crate::GenericDrawable;
use crate::GenericLayer;
use std::borrow::Cow;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Zeroable, bytemuck::Pod)]
pub struct Circle {
    pub position: [f32; 2],
    pub color: Color,
    pub radius: f32,
}

impl GpuSerializable for Circle {
    fn gpu_serialize(data: &[Self]) -> &[u8] {
        bytemuck::cast_slice(data)
    }

    fn buffer_layout<'a>() -> VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
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
                    format: wgpu::VertexFormat::Unorm8x4,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32,
                },
            ],
        }
    }
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
    instance_buffer: GpuBuffer<Circle>,
}

impl Drawable for CirclesLayerDrawable {
    fn draw<'a>(&'a self, draw_state: &DrawState<'a>) {
        let mut render_pass = draw_state.render_pass.borrow_mut();
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, draw_state.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.instance_buffer.all());
        render_pass.draw(0..6, 0..self.instance_buffer.len());
    }
}

impl GenericLayer for CirclesLayer {
    fn init_drawable_generic(&self, draw_context: &DrawContext) -> crate::GenericDrawable {
        GenericDrawable::new(self.init_drawable(draw_context))
    }
}

impl Layer for CirclesLayer {
    type D = CirclesLayerDrawable;

    fn init_drawable(&self, draw_context: &DrawContext) -> CirclesLayerDrawable {
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
                buffers: &[Circle::buffer_layout()],
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

        CirclesLayerDrawable {
            render_pipeline,
            instance_buffer,
        }
    }
}
