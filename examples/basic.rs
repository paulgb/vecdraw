use vecdraw::{
    run_event_loop, Circle, CirclesLayer, DrawContext,
    GridLayer, Layer, Line, LinesLayer, Rectangle,
    RectanglesLayer, GroupLayerDrawable
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
            grid: GridLayer::new(5, 5, palette::named::PALETURQUOISE.into()),
            circles1: CirclesLayer::new(vec![
                Circle {
                    position: [-220., -220.],
                    radius: 15.,
                    color: palette::named::PURPLE.into(),
                },
                Circle {
                    position: [300., 300.],
                    radius: 50.,
                    color: palette::named::SADDLEBROWN.into(),
                },
                Circle {
                    position: [-350., -350.],
                    radius: 70.,
                    color: palette::named::STEELBLUE.into(),
                },
            ]),
            circles2: CirclesLayer::new(vec![Circle {
                position: [500., -300.],
                radius: 40.,
                color: palette::named::VIOLET.into(),
            }]),
            rects: RectanglesLayer::new(vec![
                Rectangle {
                    upper_left: [-400., 400.],
                    bottom_right: [-450., 500.],
                    color: palette::named::SALMON.into(),
                },
                Rectangle {
                    upper_left: [10., 250.],
                    bottom_right: [50., 300.],
                    color: palette::named::MAROON.into(),
                },
            ]),
            lines: LinesLayer::new(vec![
                Line {
                    start: [450., -450.],
                    end: [200., -100.],
                    width: 5.,
                    color: palette::named::LIGHTPINK.into(),
                },
                Line {
                    start: [-450., -450.],
                    end: [200., -100.],
                    width: 30.,
                    color: palette::named::TOMATO.into(),
                },
            ]),
        }
    }
}

impl Layer for BasicApp {
    type D = GroupLayerDrawable;

    fn init_drawable(&self, draw_context: &DrawContext) -> GroupLayerDrawable {
        GroupLayerDrawable::new(vec![
            Box::new(self.grid.init_drawable(draw_context)),
            Box::new(self.circles1.init_drawable(draw_context)),
            Box::new(self.circles2.init_drawable(draw_context)),
            Box::new(self.rects.init_drawable(draw_context)),
            Box::new(self.lines.init_drawable(draw_context)),
        ])
    }
}

fn main() {
    let layer = BasicApp::new();
    run_event_loop(layer);
}
