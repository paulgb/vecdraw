use clap::Clap;
use vecdraw::{run_event_loop, Circle, CirclesLayer, Layer, CirclesLayerDrawable};
use wgpu::{Device, BindGroupLayout, SwapChainDescriptor};

#[derive(Clap)]
struct Opts {
    #[clap(default_value = "10")]
    rows: u32,
    #[clap(default_value = "10")]
    cols: u32,
}

struct GridDrawable(u32, u32);

impl Layer for GridDrawable {
    type D = CirclesLayerDrawable;

    fn init_drawable(&self, device: &Device, sc_desc: &SwapChainDescriptor, transform_layout: &BindGroupLayout) -> Self::D {
        let GridDrawable(rows, cols) = *self;
        let circles: Vec<Circle> = (0..rows)
            .into_iter()
            .flat_map(|r| {
                (0..cols).into_iter().map(move |c| Circle {
                    position: [c as f32 * 20., r as f32 * 20.],
                    radius: 10.,
                    color: [0.3, 0.2, 0.1, 1.0],
                })
            })
            .collect();

        CirclesLayer::new(circles).init_drawable(device, sc_desc, transform_layout)
    }
}

fn main() {
    let opts = Opts::parse();

    let layer = GridDrawable(opts.rows, opts.cols);
    run_event_loop(layer);
}
