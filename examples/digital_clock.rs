use chrono::{Local, Timelike};
use vecdraw::{Circle, CirclesLayer, CirclesLayerDrawable, Color, Drawable, Layer, UpdateState, run_event_loop};

struct IndicatorLayer {
    offset: f32,
    color: Color,
}

impl Layer for IndicatorLayer {
    type D = IndicatorDrawable;

    fn init_drawable(&self, draw_context: &vecdraw::DrawContext) -> Self::D {
        IndicatorDrawable {
            new_value: 0,
            last_value: 0,
            offset: self.offset,
            circles: CirclesLayer::new(vec![]).init_drawable(draw_context),
            color: self.color,
        }
    }
}

struct IndicatorDrawable {
    new_value: u32,
    last_value: u32,
    offset: f32,
    circles: CirclesLayerDrawable,
    color: Color,
}

impl Drawable for IndicatorDrawable {
    fn update(&mut self, update_state: &UpdateState) {
        if self.new_value != self.last_value {
            let circles: Vec<Circle> = (0..self.new_value).map(|c| Circle {
                color: self.color,
                position: [(c % 6) as f32 * 20., self.offset + (c / 6) as f32 * 20.],
                radius: 10.,
            }).collect();

            self.circles.instance_buffer.update(&circles, update_state.device,
                update_state.encoder.borrow_mut());
            self.last_value = self.new_value;
        }
    }

    fn draw<'a>(&'a self, draw_state: &vecdraw::DrawState<'a>) {
        self.circles.draw(draw_state);
    }
}

impl IndicatorDrawable {
    pub fn set_value(&mut self, v: u32) {
        self.new_value = v;
    }
}

struct ClockLayer;

impl Layer for ClockLayer {
    type D = ClockDrawable;

    fn init_drawable(&self, draw_context: &vecdraw::DrawContext) -> Self::D {
        ClockDrawable {
            seconds: IndicatorLayer {
                color: palette::named::RED.into(),
                offset: -300.,
            }.init_drawable(draw_context),
            minutes: IndicatorLayer {
                color: palette::named::GREEN.into(),
                offset: 0.,
            }.init_drawable(draw_context),
            hours: IndicatorLayer {
                color: palette::named::BLUE.into(),
                offset: 300.,
            }.init_drawable(draw_context)
        }
    }
}

struct ClockDrawable {
    seconds: IndicatorDrawable,
    minutes: IndicatorDrawable,
    hours: IndicatorDrawable,
}

impl Drawable for ClockDrawable {
    fn update(&mut self, update_state: &UpdateState) {
        let now = Local::now();
        self.seconds.set_value(now.second());
        self.seconds.update(update_state);

        self.minutes.set_value(now.minute());
        self.minutes.update(update_state);

        self.hours.set_value(now.hour());
        self.hours.update(update_state);
    }

    fn draw<'a>(&'a self, draw_state: &vecdraw::DrawState<'a>) {
        self.seconds.draw(draw_state);
        self.minutes.draw(draw_state);
        self.hours.draw(draw_state);
    }
}

fn main() {
    let layer = ClockLayer;
    run_event_loop(layer);
}