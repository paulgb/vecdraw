use clap::Clap;
use vecdraw::{run_event_loop, Circle, CirclesLayer, Layer, Rectangle, RectanglesLayer};

#[derive(Clap)]
struct Opts {
    #[clap(default_value = "10")]
    rows: u32,
    #[clap(default_value = "10")]
    cols: u32,
}

fn main() {
    let opts = Opts::parse();

    let rects: Vec<Circle> = (0..opts.rows)
        .into_iter()
        .flat_map(|r| {
            (0..opts.cols).into_iter().map(move |c| Circle {
                position: [c as f32 * 20., r as f32 * 20.],
                radius: 10.,
                color: [0.3, 0.2, 0.1, 1.0],
            })
        })
        .collect();

    let layers: Vec<Box<dyn Layer>> = vec![Box::new(CirclesLayer::new(rects))];

    run_event_loop(layers);
}
