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
    fn update(&mut self, _update_state: &UpdateState) {}

    fn draw<'a>(&'a self, draw_state: &DrawState<'a>);
}

impl Drawable for GroupLayerDrawable {
    fn draw<'a>(&'a self, draw_state: &DrawState<'a>) {
        for drawable in &self.drawables {
            drawable.draw(draw_state);
        }
    }
}

pub struct GenericDrawable {
    drawable: Box<dyn Drawable>,
}

impl GenericDrawable {
    pub fn new(d: impl Drawable + 'static) -> Self {
        GenericDrawable {
            drawable: Box::new(d),
        }
    }
}

impl Drawable for GenericDrawable {
    fn draw<'a>(&'a self, draw_state: &DrawState<'a>) {
        self.drawable.draw(draw_state);
    }
}

pub trait GenericLayer {
    fn init_drawable_generic(&self, draw_context: &DrawContext) -> GenericDrawable;
}

pub struct GroupLayer {
    layers: Vec<Box<dyn GenericLayer>>,
}

impl Layer for GroupLayer {
    type D = GroupLayerDrawable;

    fn init_drawable(&self, draw_context: &DrawContext) -> Self::D {
        GroupLayerDrawable {
            drawables: self
                .layers
                .iter()
                .map(|d| d.init_drawable_generic(draw_context))
                .collect(),
        }
    }
}

pub struct GroupLayerDrawable {
    drawables: Vec<GenericDrawable>,
}

impl GroupLayer {
    pub fn new(layers: Vec<Box<dyn GenericLayer>>) -> Self {
        GroupLayer { layers }
    }
}
