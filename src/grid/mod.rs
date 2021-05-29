use crate::{Hairline, HairlinesLayer, LinesLayer, Orientation};
use crate::layer::{Drawable, Layer};
use crate::line::LinesLayerDrawable;
use wgpu::util::DeviceExt;
use wgpu::{
    BindGroup, BindGroupLayout, BlendComponent, BlendState, Buffer, Device, RenderPass,
    RenderPipeline, SwapChainDescriptor,
};

type Color = [f32; 4];

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
            color: [0.9, 0.9, 0.9, 1.0],
        }
    }
}

impl Layer for GridLayer {
    fn init_drawable(
        &self,
        device: &Device,
        sc_desc: &SwapChainDescriptor,
        transform_layout: &BindGroupLayout,
    ) -> Box<dyn Drawable> {
        let SwapChainDescriptor {height, width, ..} = sc_desc;

        let offset_x = -(*width as f32) + (*width as f32 * 2. / self.cols as f32);
        let offset_y = -(*height as f32) + (*height as f32 * 2. / self.rows as f32);
        let mut grid = (0..self.rows)
            .map(|r| {
                Hairline {
                    orientation: Orientation::Horizontal,
                    width: 0.004,
                    color: self.color,
                    location: offset_y + (r as f32) * (2. * *height as f32 / (self.rows + 1) as f32),
                }
            })
            .collect::<Vec<_>>();
        let cols = (0..self.cols)
            .map(|c| {
                Hairline {
                    orientation: Orientation::Vertical,
                    width: 0.004,
                    color: self.color,
                    location: offset_x + (c as f32) * (2. * *width as f32 / (self.cols + 1) as f32),
                }
            })
            .collect::<Vec<_>>();

        grid.extend(cols);

        let lines = HairlinesLayer::new(grid);
        
        lines.init_drawable(device, sc_desc, transform_layout)
    }
}