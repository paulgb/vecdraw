use std::cell::RefCell;
use wgpu::{BindGroup, BindGroupLayout, Device, RenderPass, SwapChainDescriptor};

pub struct DrawContext<'a> {
    pub device: &'a Device,
    pub sc_desc: &'a SwapChainDescriptor,
    pub transform_layout: &'a BindGroupLayout,
}

pub trait Layer {
    type D: Drawable;

    fn init_drawable(&self, draw_context: &DrawContext) -> Self::D;
}

pub struct DrawState<'a> {
    pub render_pass: RefCell<RenderPass<'a>>,
    pub bind_group: &'a BindGroup,
}

pub trait Drawable {
    fn draw<'a>(&'a self, draw_state: &DrawState<'a>);
}
