use wgpu::{BindGroup, BindGroupLayout, Device, RenderPass, SwapChainDescriptor};
use std::cell::RefCell;

pub trait Layer {
    fn init_drawable(
        &self,
        device: &Device,
        sc_desc: &SwapChainDescriptor,
        transform_layout: &BindGroupLayout,
    ) -> Box<dyn Drawable>;
}

pub struct DrawState<'a> {
    pub render_pass: RefCell<RenderPass<'a>>,
    pub bind_group: &'a BindGroup,
}

pub trait Drawable {
    fn draw<'a>(&'a self, draw_state: &DrawState<'a>);
}
