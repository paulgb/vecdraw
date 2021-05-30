use chrono::Timelike;
use vecdraw::{
    run_event_loop, Circle, CirclesLayer, CirclesLayerDrawable, DrawContext, DrawState, Drawable,
    Layer, Line, LinesLayer, LinesLayerDrawable, UpdateState,
};

struct ClockLayer;

impl Layer for ClockLayer {
    type D = ClockApp;

    fn init_drawable(&self, draw_context: &DrawContext) -> Self::D {
        ClockApp::new(draw_context)
    }
}

struct ClockApp {
    markers: CirclesLayerDrawable,
    hands: LinesLayerDrawable,
}

impl Drawable for ClockApp {
    fn update(&self, update_state: &UpdateState) {
        self.hands.instance_buffer.update(
            &Self::hand_data(),
            update_state.device,
            update_state.encoder.borrow_mut(),
        );
    }

    fn draw<'a>(&'a self, draw_state: &DrawState<'a>) {
        self.markers.draw(draw_state);
        self.hands.draw(draw_state);
    }
}

impl ClockApp {
    fn hand_data() -> Vec<Line> {
        let time = chrono::Local::now();

        let mili = time.nanosecond() as f32 / 1e9;
        let second = time.second() as f32 + mili;

        let second_theta = std::f32::consts::TAU * (second / 60.);

        let minute: f32 = time.minute() as f32 + second / 60.;
        let minute_theta = std::f32::consts::TAU * (minute / 60.);

        let hour: f32 = time.hour12().1 as f32 + minute / 60.;
        let hour_theta = std::f32::consts::TAU * (hour / 12.);

        let hour_hand = {
            let hour_hand_len = 300.;

            Line {
                start: [0., 0.],
                end: [
                    hour_hand_len * hour_theta.sin(),
                    hour_hand_len * hour_theta.cos(),
                ],
                color: palette::named::DARKOLIVEGREEN.into(),
                width: 10.,
            }
        };

        let minute_hand = {
            let minute_hand_len = 350.;

            Line {
                start: [0., 0.],
                end: [
                    minute_hand_len * minute_theta.sin(),
                    minute_hand_len * minute_theta.cos(),
                ],
                color: palette::named::DODGERBLUE.into(),
                width: 5.,
            }
        };

        let second_hand = {
            let second_hand_len = 380.;

            Line {
                start: [0., 0.],
                end: [
                    second_hand_len * second_theta.sin(),
                    second_hand_len * second_theta.cos(),
                ],
                color: palette::named::LIGHTSLATEGRAY.into(),
                width: 3.,
            }
        };

        vec![hour_hand, minute_hand, second_hand]
    }

    fn new(draw_context: &DrawContext) -> Self {
        let markers = CirclesLayer::new(
            (0..12)
                .map(|h| {
                    let theta = std::f32::consts::TAU * (h as f32) / 12.;
                    let r = 400.;

                    Circle {
                        radius: 20.,
                        color: palette::named::DARKCYAN.into(),
                        position: [r * theta.sin(), r * theta.cos()],
                    }
                })
                .collect(),
        );

        let hands = LinesLayer::new(Self::hand_data());

        ClockApp {
            hands: hands.init_drawable(draw_context),
            markers: markers.init_drawable(draw_context),
        }
    }
}

fn main() {
    let layer = ClockLayer;
    run_event_loop(layer);
}
