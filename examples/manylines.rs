use clap::Clap;
use rand::Rng;
use vecdraw::{run_event_loop, DrawContext, Layer, Line, LinesLayer, LinesLayerDrawable};

const EXTENT: f32 = 10000.;
const MAX_LEN: f32 = 1000.;

#[derive(Clap)]
struct Opts {
    #[clap(default_value = "100")]
    lines: u32,
}

struct ManyLines(u32);

impl Layer for ManyLines {
    type D = LinesLayerDrawable;

    fn init_drawable(&self, draw_context: &DrawContext) -> Self::D {
        let mut rand = rand::thread_rng();
        let lines: Vec<Line> = (0..self.0)
            .into_iter()
            .map(|_| {
                let start = [
                    rand.gen_range(-EXTENT..EXTENT),
                    rand.gen_range(-EXTENT..EXTENT),
                ];
                let end = [
                    start[0] + rand.gen_range(-MAX_LEN..MAX_LEN),
                    start[1] + rand.gen_range(-MAX_LEN..MAX_LEN),
                ];

                Line {
                    color: [
                        rand.gen_range(0.0..1.0),
                        rand.gen_range(0.0..1.0),
                        rand.gen_range(0.0..1.0),
                        1.0,
                    ],
                    start,
                    end,
                    width: rand.gen_range(1.0..100.0),
                }
            })
            .collect();

        LinesLayer::new(lines).init_drawable(draw_context)
    }
}

fn main() {
    let opts = Opts::parse();

    let layer = ManyLines(opts.lines);
    run_event_loop(layer);
}
