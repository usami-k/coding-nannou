use nannou::color::*;
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    start_animation: bool,
    rotation: f32,
    step: f32,
    zoom: f32,
    current_angle: f32,
    current_start: Point2,
    current_end: Point2,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(600, 600)
        .key_pressed(key_pressed)
        .view(view)
        .build()
        .unwrap();
    Model {
        _window,
        start_animation: false,
        rotation: 2.0 * PI * 17.0 / 55.0,
        step: 0.1,
        zoom: 40.0,
        current_angle: 0.0,
        current_start: pt2(0.0, 0.0),
        current_end: pt2(0.0, 0.0),
    }
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    if key == Key::Space {
        model.start_animation = !model.start_animation
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if !model.start_animation {
        return;
    }

    model.current_start = model.current_end;
    model.current_angle += model.step;
    let radius = (model.current_angle / model.rotation).sqrt();
    let point = pt2(model.current_angle.cos(), model.current_angle.sin()) * radius;
    model.current_end = point * model.zoom;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    let text_position = app.window_rect().mid_top() + vec2(0.0, -20.0);
    let text_width = app.window_rect().w();
    draw.text("Fermat's Spiral: r = sqrt(θ/a)")
        .xy(text_position)
        .w(text_width)
        .font_size(18)
        .color(BLACK);

    if app.elapsed_frames() == 0 {
        draw.background().color(WHITE);
    }

    draw.line()
        .start(model.current_start)
        .end(model.current_end)
        .color(STEELBLUE)
        .weight(2.0);

    draw.to_frame(app, &frame).unwrap();
}
