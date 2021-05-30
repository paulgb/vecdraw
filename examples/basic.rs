use vecdraw::{
    run_event_loop, Circle, CirclesLayer, GridLayer, Layer, Line, LinesLayer, Rectangle,
    RectanglesLayer,
};

fn main() {
    let layers: Vec<Box<dyn Layer>> = vec![
        Box::new(GridLayer::new(5, 5, [0.4, 0.8, 0.7, 1.0])),
        Box::new(CirclesLayer::new(vec![
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
        ])),
        Box::new(CirclesLayer::new(vec![Circle {
            position: [500., -300.],
            radius: 40.,
            color: [0.3, 0.6, 0.9, 1.],
        }])),
        Box::new(RectanglesLayer::new(vec![
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
        ])),
        Box::new(LinesLayer::new(vec![
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
        ])),
    ];

    run_event_loop(layers);
}
