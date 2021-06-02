use crate::layer::{DrawContext, DrawState, Drawable, Layer};
use crate::{GenericDrawable, GenericLayer};

use crate::color::Color;
use crate::gpu_data::{GpuBuffer, GpuSerializable};
use std::borrow::Cow;
use wgpu::{
    BlendComponent, BlendState, RenderPipeline, ShaderModuleDescriptor, ShaderSource,
    VertexBufferLayout,
};

#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum Orientation {
    Vertical = 0,
    Horizontal = 1,
}

unsafe impl bytemuck::Zeroable for Orientation {}
unsafe impl bytemuck::Pod for Orientation {}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Zeroable, bytemuck::Pod)]
pub struct Hairline {
    pub location: f32,
    pub color: Color,
    pub width: f32,
    pub orientation: Orientation,
}

impl GpuSerializable for Hairline {
    fn gpu_serialize(data: &[Self]) -> &[u8] {
        bytemuck::cast_slice(data)
    }

    fn buffer_layout<'a>() -> VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Hairline>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 1]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Unorm8x4,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Uint32,
                },
            ],
        }
    }
}

pub struct HairlinesLayer {
    data: Vec<Hairline>,
}

impl HairlinesLayer {
    pub fn new(data: Vec<Hairline>) -> Self {
        HairlinesLayer { data }
    }
}

pub struct HairlinesLayerDrawable {
    render_pipeline: RenderPipeline,
    instance_buffer: GpuBuffer<Hairline>,
}

impl Drawable for HairlinesLayerDrawable {
    fn draw<'a>(&'a self, draw_state: &DrawState<'a>) {
        let mut render_pass = draw_state.render_pass.borrow_mut();
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, draw_state.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.instance_buffer.all());
        render_pass.draw(0..6, 0..self.instance_buffer.len());
    }
}

impl Layer for HairlinesLayer {
    type D = HairlinesLayerDrawable;

    fn init_drawable(&self, draw_context: &DrawContext) -> HairlinesLayerDrawable {
        let DrawContext {
            device,
            sc_desc,
            transform_layout,
        } = *draw_context;
        let instance_buffer = GpuBuffer::new(&self.data, &device);

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
                buffers: &[Hairline::buffer_layout()],
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

        HairlinesLayerDrawable {
            render_pipeline,
            instance_buffer,
        }
    }
}

impl GenericLayer for HairlinesLayer {
    fn init_drawable_generic(&self, draw_context: &DrawContext) -> crate::GenericDrawable {
        GenericDrawable::new(self.init_drawable(draw_context))
    }
}
