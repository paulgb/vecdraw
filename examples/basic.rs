use vecdraw::{
    run_event_loop, Circle, CirclesLayer, GridLayer, GroupLayer, Line, LinesLayer, Rectangle,
    RectanglesLayer,
};

fn create() -> GroupLayer {
    GroupLayer::new(vec![
        Box::new(GridLayer::new(5, 5, palette::named::PALETURQUOISE.into())),
        Box::new(CirclesLayer::new(vec![
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
        ])),
        Box::new(CirclesLayer::new(vec![Circle {
            position: [500., -300.],
            radius: 40.,
            color: palette::named::VIOLET.into(),
        }])),
        Box::new(RectanglesLayer::new(vec![
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
        ])),
        Box::new(LinesLayer::new(vec![
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
        ])),
    ])
}

fn main() {
    let layer = create();
    run_event_loop(layer);
}
