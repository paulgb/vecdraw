use std::cell::RefCell;
use wgpu::{BindGroup, BindGroupLayout, CommandEncoder, Device, RenderPass, SwapChainDescriptor};

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

pub struct UpdateState<'a> {
    pub encoder: &'a RefCell<CommandEncoder>,
    pub device: &'a Device,
}

pub trait Drawable {
    fn update(&self, _update_state: &UpdateState) {}

    fn draw<'a>(&'a self, draw_state: &DrawState<'a>);
}

pub struct GroupLayerDrawable {
    drawables: Vec<Box<dyn Drawable>>,
}

impl GroupLayerDrawable {
    pub fn new(drawables: Vec<Box<dyn Drawable>>) -> Self {
        GroupLayerDrawable { drawables }
    }
}

impl Drawable for GroupLayerDrawable {
    fn draw<'a>(&'a self, draw_state: &DrawState<'a>) {
        for drawable in &self.drawables {
            drawable.draw(draw_state);
        }
    }
}
