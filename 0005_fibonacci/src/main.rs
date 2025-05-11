use nannou::color::*;
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Square {
    left: f32,
    bottom: f32,
    length: f32,
}

struct Model {
    _window: window::Id,
    start_animation: bool,
    time: f32,
    current1: usize,
    current2: usize,
    squares: Vec<Square>,
    scale: f32,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .key_pressed(key_pressed)
        .view(view)
        .build()
        .unwrap();
    Model {
        _window,
        start_animation: false,
        time: 0.0,
        current1: 0,
        current2: 0,
        squares: Vec::new(),
        scale: 20.0,
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
    if app.time < model.time + 1.0 {
        return;
    }
    model.time = app.time;

    if model.current2 == 0 {
        model.current2 = 1;
        let square = Square {
            left: 0.0,
            bottom: 0.0,
            length: 1.0,
        };
        model.squares.push(square);
        return;
    }

    if model.current1 < model.current2 {
        model.current1 += model.current2;
        let square = Square {
            left: model.current2 as f32,
            bottom: 0.0,
            length: model.current1 as f32,
        };
        model.squares.push(square);
    } else {
        model.current2 += model.current1;
        let square = Square {
            left: 0.0,
            bottom: model.current1 as f32,
            length: model.current2 as f32,
        };
        model.squares.push(square);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);

    let origin_x = app.window_rect().left() + 4.0;
    let origin_y = app.window_rect().bottom() + 4.0;

    for square in &model.squares {
        let center_x = (square.left + square.length / 2.0) * model.scale;
        let center_y = (square.bottom + square.length / 2.0) * model.scale;
        let length = square.length * model.scale;
        draw.rect()
            .x_y(center_x + origin_x, center_y + origin_y)
            .w_h(length, length)
            .color(LIGHTCYAN)
            .stroke_weight(1.0)
            .stroke_color(BLACK);
    }

    draw.to_frame(app, &frame).unwrap();
}
