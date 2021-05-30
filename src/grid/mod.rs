use crate::layer::{DrawContext, Layer};
use crate::{Hairline, HairlinesLayer, HairlinesLayerDrawable, Orientation};

use crate::color::Color;
use wgpu::SwapChainDescriptor;

pub struct GridLayer {
    rows: u32,
    cols: u32,
    color: Color,
}

impl GridLayer {
    pub fn new(rows: u32, cols: u32, color: Color) -> Self {
        Self { rows, cols, color }
    }
}

impl Default for GridLayer {
    fn default() -> Self {
        Self {
            rows: 10,
            cols: 10,
            color: Color(0x000000ff),
        }
    }
}

impl Layer for GridLayer {
    type D = HairlinesLayerDrawable;

    fn init_drawable(&self, draw_context: &DrawContext) -> HairlinesLayerDrawable {
        let DrawContext {
            device: _,
            sc_desc,
            transform_layout: _,
        } = *draw_context;
        let SwapChainDescriptor { height, width, .. } = sc_desc;

        let offset_x = -(*width as f32) + (*width as f32 * 2. / self.cols as f32);
        let offset_y = -(*height as f32) + (*height as f32 * 2. / self.rows as f32);
        let mut grid = (0..self.rows)
            .map(|r| Hairline {
                orientation: Orientation::Horizontal,
                width: 0.004,
                color: self.color,
                location: offset_y + (r as f32) * (2. * *height as f32 / (self.rows + 1) as f32),
            })
            .collect::<Vec<_>>();
        let cols = (0..self.cols)
            .map(|c| Hairline {
                orientation: Orientation::Vertical,
                width: 0.004,
                color: self.color,
                location: offset_x + (c as f32) * (2. * *width as f32 / (self.cols + 1) as f32),
            })
            .collect::<Vec<_>>();

        grid.extend(cols);

        let lines = HairlinesLayer::new(grid);

        lines.init_drawable(draw_context)
    }
}
