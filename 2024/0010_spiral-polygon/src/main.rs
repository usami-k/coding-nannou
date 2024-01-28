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
    num: usize,
    vertices: Vec<Point2>,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(420, 420)
        .key_pressed(key_pressed)
        .view(view)
        .build()
        .unwrap();
    let num = 11;
    let vertices = (0..num)
        .map(|i| {
            let angle = PI * 2.0 / num as f32 * i as f32;
            let x = angle.cos() * 200.0;
            let y = angle.sin() * 200.0;
            pt2(x, y)
        })
        .collect();
    Model {
        _window,
        start_animation: false,
        updated_time: 0.0,
        num,
        vertices,
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

    let vertices = (0..model.num)
        .map(|i| {
            let next = (i + 1) % model.num;
            model.vertices[i] + model.vertices[next].sub(model.vertices[i]) * 0.23
        })
        .collect();
    model.vertices = vertices;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    if app.elapsed_frames() == 0 {
        draw.background().color(WHITE);
    }

    draw.polyline()
        .points_closed(model.vertices.clone())
        .color(STEELBLUE);

    draw.to_frame(app, &frame).unwrap();
}
