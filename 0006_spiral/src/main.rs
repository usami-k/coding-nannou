use nannou::color::*;
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    start_animation: bool,
    angle: f32,
    start: Point2,
    end: Point2,
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
        angle: 0.0,
        start: pt2(0.0, 0.0),
        end: pt2(0.0, 0.0),
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

    model.start = model.end;
    model.angle += PI / 30.0;
    let radius = 1.1.pow(model.angle) as f32;
    model.end = pt2(model.angle.cos(), model.angle.sin()) * radius;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    if app.elapsed_frames() == 0 {
        draw.background().color(WHITE);
    }

    draw.line()
        .start(model.start)
        .end(model.end)
        .color(STEELBLUE)
        .weight(2.0);

    draw.to_frame(app, &frame).unwrap();
}
