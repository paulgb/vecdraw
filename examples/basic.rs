use vecdraw::{
    run_event_loop, Circle, CirclesLayer, CirclesLayerDrawable, DrawContext, DrawState, Drawable,
    GridLayer, HairlinesLayerDrawable, Layer, Line, LinesLayer, LinesLayerDrawable, Rectangle,
    RectanglesLayer, RectanglesLayerDrawable,
};

struct BasicApp {
    grid: GridLayer,
    circles1: CirclesLayer,
    circles2: CirclesLayer,
    rects: RectanglesLayer,
    lines: LinesLayer,
}

impl BasicApp {
    pub fn new() -> BasicApp {
        BasicApp {
            grid: GridLayer::new(5, 5, [0.4, 0.8, 0.7, 1.0]),
            circles1: CirclesLayer::new(vec![
                Circle {
                    position: [-220., -220.],
                    radius: 15.,
                    color: [0.1, 1.0, 0.5, 1.],
                },
                Circle {
                    position: [300., 300.],
                    radius: 50.,
                    color: [0.6, 0.6, 0., 1.],
                },
                Circle {
                    position: [-350., -350.],
                    radius: 70.,
                    color: [0.7, 0., 0.4, 1.],
                },
            ]),
            circles2: CirclesLayer::new(vec![Circle {
                position: [500., -300.],
                radius: 40.,
                color: [0.3, 0.6, 0.9, 1.],
            }]),
            rects: RectanglesLayer::new(vec![
                Rectangle {
                    upper_left: [-400., 400.],
                    bottom_right: [-450., 500.],
                    color: [0.3, 0.6, 0.4, 1.],
                },
                Rectangle {
                    upper_left: [10., 250.],
                    bottom_right: [50., 300.],
                    color: [0.7, 0., 0.4, 1.],
                },
            ]),
            lines: LinesLayer::new(vec![
                Line {
                    start: [450., -450.],
                    end: [200., -100.],
                    width: 5.,
                    color: [0.5, 0.0, 0.0, 1.0],
                },
                Line {
                    start: [-450., -450.],
                    end: [200., -100.],
                    width: 30.,
                    color: [0.0, 0.5, 0.5, 1.0],
                },
            ]),
        }
    }
}

impl Layer for BasicApp {
    type D = BasicAppDrawable;

    fn init_drawable(&self, draw_context: &DrawContext) -> BasicAppDrawable {
        BasicAppDrawable {
            grid_drawable: self.grid.init_drawable(draw_context),
            circles1_drawable: self.circles1.init_drawable(draw_context),
            circles2_drawable: self.circles2.init_drawable(draw_context),
            rects_drawable: self.rects.init_drawable(draw_context),
            lines_drawable: self.lines.init_drawable(draw_context),
        }
    }
}

struct BasicAppDrawable {
    grid_drawable: HairlinesLayerDrawable,
    circles1_drawable: CirclesLayerDrawable,
    circles2_drawable: CirclesLayerDrawable,
    rects_drawable: RectanglesLayerDrawable,
    lines_drawable: LinesLayerDrawable,
}

impl Drawable for BasicAppDrawable {
    fn draw<'a>(&'a self, draw_state: &DrawState<'a>) {
        self.grid_drawable.draw(draw_state);
        self.circles1_drawable.draw(draw_state);
        self.circles2_drawable.draw(draw_state);
        self.rects_drawable.draw(draw_state);
        self.lines_drawable.draw(draw_state);
    }
}

fn main() {
    let layer = BasicApp::new();
    run_event_loop(layer);
}
