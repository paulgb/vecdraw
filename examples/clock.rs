use chrono::Timelike;
use vecdraw::{run_event_loop, Circle, CirclesLayer, Drawable, Layer, Line, LinesLayer};

struct ClockApp {
    markers: Box<dyn Drawable>,
    hands: Box<dyn Drawable>,
}

impl Drawable for ClockApp {
    fn draw<'a>(&'a self, draw_state: &DrawState<'a>) {
        self.markers.draw(draw_state);
        self.hands.draw(draw_state);
    }
}

impl ClockApp {
    fn new() -> Self {
        let markers = CirclesLayer::new(
            (0..12)
                .map(|h| {
                    let theta = std::f32::consts::TAU * (h as f32) / 12.;
                    let r = 400.;

                    Circle {
                        radius: 20.,
                        color: [0.5, 0.3, 0.1, 1.0],
                        position: [r * theta.sin(), r * theta.cos()],
                    }
                })
                .collect(),
        );

        let time = chrono::Local::now();

        let (_, hour) = time.hour12();
        let minute = time.minute();
        let second = time.second();

        let hour_hand = {
            let hour_frac = hour as f32 + (minute as f32 / 60.);
            let hour_theta = std::f32::consts::TAU * hour_frac / 12.;
            let hour_hand_len = 300.;

            Line {
                start: [0., 0.],
                end: [
                    hour_hand_len * hour_theta.sin(),
                    hour_hand_len * hour_theta.cos(),
                ],
                color: [0.2, 0.4, 0.1, 1.0],
                width: 10.,
            }
        };

        let minute_hand = {
            let minute_theta = std::f32::consts::TAU * minute as f32 / 60.;
            let minute_hand_len = 350.;

            Line {
                start: [0., 0.],
                end: [
                    minute_hand_len * minute_theta.sin(),
                    minute_hand_len * minute_theta.cos(),
                ],
                color: [0.1, 0.2, 0.0, 1.0],
                width: 5.,
            }
        };

        let second_hand = {
            let second_theta = std::f32::consts::TAU * second as f32 / 60.;
            let second_hand_len = 380.;

            Line {
                start: [0., 0.],
                end: [
                    second_hand_len * second_theta.sin(),
                    second_hand_len * second_theta.cos(),
                ],
                color: [0.1, 0.2, 0.0, 1.0],
                width: 3.,
            }
        };

        let hands = LinesLayer::new(vec![hour_hand, minute_hand, second_hand]);

        ClockApp {
            hands: Box::new(hands.init_drawable()),
            markers,
        }
    }
}

fn main() {
    let layers: Vec<Box<dyn Layer>> = vec![Box::new(markers), Box::new(hands)];

    run_event_loop(layers);
}
