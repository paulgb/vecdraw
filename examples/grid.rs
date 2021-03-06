use clap::Parser;
use vecdraw::{run_event_loop, Circle, CirclesLayer, CirclesLayerDrawable, DrawContext, Layer};

#[derive(Parser)]
struct Opts {
    #[clap(default_value = "10")]
    rows: u32,
    #[clap(default_value = "10")]
    cols: u32,
}

struct GridDrawable(u32, u32);

impl Layer for GridDrawable {
    type D = CirclesLayerDrawable;

    fn init_drawable(&self, draw_context: &DrawContext) -> Self::D {
        let GridDrawable(rows, cols) = *self;
        let circles: Vec<Circle> = (0..rows)
            .into_iter()
            .flat_map(|r| {
                (0..cols).into_iter().map(move |c| Circle {
                    position: [c as f32 * 20., r as f32 * 20.],
                    radius: 10.,
                    color: palette::named::RED.into(),
                })
            })
            .collect();

        CirclesLayer::new(circles).init_drawable(draw_context)
    }
}

fn main() {
    let opts = Opts::parse();

    let layer = GridDrawable(opts.rows, opts.cols);
    run_event_loop(layer);
}
