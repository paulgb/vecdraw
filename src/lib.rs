use std::iter;

use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferDescriptor, BufferUsage,
    ShaderStage,
};
use winit::dpi::PhysicalSize;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

pub use circle::{Circle, CirclesLayer};
pub use layer::{Drawable, Layer};
use zoom::ZoomState;

pub use crate::line::{Line, LinesLayer};
pub use crate::rectangle::{Rectangle, RectanglesLayer};
pub use crate::hairline::{Hairline, HairlinesLayer, HairlinesLayerDrawable, Orientation};
pub use crate::grid::{GridLayer};
use crate::zoom::Mat4;

mod circle;
mod layer;
mod line;
mod rectangle;
mod zoom;
mod grid;
mod hairline;

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    size: winit::dpi::PhysicalSize<u32>,
    transform_buffer: Buffer,
    transform_bind_group: BindGroup,

    drawables: Vec<Box<dyn Drawable>>,
    zoom_state: ZoomState,
}

impl State {
    async fn new(window: &Window, layers: Vec<Box<dyn Layer>>) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None, // Trace path
            )
            .await
            .unwrap();

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Mailbox,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        let zoom_state = ZoomState::new(size);
        let transform = zoom_state.matrix();

        let transform_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Transformation buffer"),
            contents: bytemuck::cast_slice(&[transform]),
            usage: BufferUsage::UNIFORM | BufferUsage::COPY_DST,
        });

        let transform_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Transformation bind group layout"),
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStage::VERTEX,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let transform_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Transformation bind group"),
            layout: &transform_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: transform_buffer.as_entire_binding(),
            }],
        });

        let drawables = layers
            .into_iter()
            .map(|d| d.init_drawable(&device, &sc_desc, &transform_layout))
            .collect();

        Self {
            surface,
            device,
            queue,
            size,
            sc_desc,
            swap_chain,
            drawables,
            transform_buffer,
            transform_bind_group,
            zoom_state,
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        self.zoom_state.set_size(new_size);

        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }

    fn input(&mut self, event: &WindowEvent, window: &Window) -> bool {
        self.zoom_state.handle_event(event, window)
    }

    fn render(&mut self) -> Result<(), wgpu::SwapChainError> {
        let frame = self.swap_chain.get_current_frame()?.output;

        let tmp_buffer = self.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Temporary Buffer"),
            contents: bytemuck::cast_slice(&self.zoom_state.matrix()),
            usage: BufferUsage::COPY_SRC,
        });

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        encoder.copy_buffer_to_buffer(
            &tmp_buffer,
            0,
            &self.transform_buffer,
            0,
            std::mem::size_of::<Mat4>() as u64,
        );

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &frame.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 1.,
                            g: 1.,
                            b: 1.,
                            a: 1.,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            for drawable in &self.drawables {
                drawable.draw(&mut render_pass, &self.transform_bind_group);
            }
        }

        self.queue.submit(iter::once(encoder.finish()));

        Ok(())
    }
}

pub fn run_event_loop(layers: Vec<Box<dyn Layer>>) {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Shape Drawing Demo")
        .with_inner_size(PhysicalSize {
            width: 600,
            height: 600,
        })
        .build(&event_loop)
        .unwrap();

    use futures::executor::block_on;

    let mut state = block_on(State::new(&window, layers));

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                if !state.input(event, &window) {
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            state.resize(*physical_size);
                            window.request_redraw();
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            state.resize(**new_inner_size);
                            window.request_redraw();
                        }
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(_) => {
                match state.render() {
                    Ok(_) => {}
                    // Recreate the swap_chain if lost
                    Err(wgpu::SwapChainError::Lost) => state.resize(state.size),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            _ => *control_flow = ControlFlow::Wait,
        }
    });
}
