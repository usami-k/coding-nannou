use nannou::color::*;
use nannou::prelude::*;
use std::ops::Sub;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    start_animation: bool,
    updated_time: f32,
    vertex1: Point2,
    vertex2: Point2,
    vertex3: Point2,
    vertex4: Point2,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(420, 420)
        .key_pressed(key_pressed)
        .view(view)
        .build()
        .unwrap();
    Model {
        _window,
        start_animation: false,
        updated_time: 0.0,
        vertex1: pt2(200.0, 200.0),
        vertex2: pt2(-200.0, 200.0),
        vertex3: pt2(-200.0, -200.0),
        vertex4: pt2(200.0, -200.0),
    }
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    if key == Key::Space {
        model.start_animation = !model.start_animation
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    if !model.start_animation {
        return;
    }

    if app.time < model.updated_time + 0.5 {
        return;
    }
    model.updated_time = app.time;

    let vertex1 = model.vertex1;
    let vertex2 = model.vertex2;
    let vertex3 = model.vertex3;
    let vertex4 = model.vertex4;
    model.vertex1 = vertex1 + vertex4.sub(vertex1) * 0.1;
    model.vertex2 = vertex2 + vertex1.sub(vertex2) * 0.1;
    model.vertex3 = vertex3 + vertex2.sub(vertex3) * 0.1;
    model.vertex4 = vertex4 + vertex3.sub(vertex4) * 0.1;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    if app.elapsed_frames() == 0 {
        draw.background().color(WHITE);
    }

    draw.polyline()
        .points(vec![
            model.vertex1,
            model.vertex2,
            model.vertex3,
            model.vertex4,
            model.vertex1,
        ])
        .color(STEELBLUE);

    draw.to_frame(app, &frame).unwrap();
}
