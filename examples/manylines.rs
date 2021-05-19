use clap::Clap;
use rand::Rng;
use vecdraw::{run_event_loop, Layer, Line, LinesLayer};

const EXTENT: f32 = 10000.;
const MAX_LEN: f32 = 1000.;

#[derive(Clap)]
struct Opts {
    #[clap(default_value = "100")]
    lines: u32,
}

fn main() {
    let opts = Opts::parse();
    let mut rand = rand::thread_rng();

    let rects: Vec<Line> = (0..opts.lines)
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

    let layers: Vec<Box<dyn Layer>> = vec![Box::new(LinesLayer::new(rects))];

    run_event_loop(layers);
}
