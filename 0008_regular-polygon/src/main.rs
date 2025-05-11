use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    start_animation: bool,
    updated_time: f32,
    num: usize,
    points: Vec<Point2>,
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
        num: 0,
        points: vec![pt2(200.0, 0.0)],
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

    if app.time < model.updated_time + 1.0 {
        return;
    }
    model.updated_time = app.time;

    model.num += 1;
    model.points = (0..=model.num)
        .map(|i| {
            let angle = PI * 2.0 / model.num as f32 * i as f32;
            let x = angle.cos() * 200.0;
            let y = angle.sin() * 200.0;
            pt2(x, y)
        })
        .collect();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);

    draw.text(&model.num.to_string())
        .color(STEELBLUE)
        .font_size(32)
        .x_y(0.0, 8.0);

    draw.polyline()
        .color(STEELBLUE)
        .weight(2.0)
        .points(model.points.clone());

    draw.to_frame(app, &frame).unwrap();
}
