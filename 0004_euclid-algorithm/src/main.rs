use nannou::color::*;
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Square {
    left: usize,
    bottom: usize,
    length: usize,
}

struct Model {
    _window: window::Id,
    start_animation: bool,
    time: f32,
    num1: usize,
    num2: usize,
    current1: usize,
    current2: usize,
    squares: Vec<Square>,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(600, 400)
        .key_pressed(key_pressed)
        .view(view)
        .build()
        .unwrap();
    let num1 = 580;
    let num2 = 340;
    Model {
        _window,
        start_animation: false,
        time: 0.0,
        num1,
        num2,
        current1: num1,
        current2: num2,
        squares: Vec::new(),
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

    if model.current1 == 0 || model.current2 == 0 {
        return;
    }

    if model.current1 >= model.current2 {
        let square = Square {
            left: model.num1 - model.current1,
            bottom: model.num2 - model.current2,
            length: model.current2,
        };
        model.squares.push(square);
        model.current1 -= model.current2;
    } else {
        let square = Square {
            left: model.num1 - model.current1,
            bottom: model.num2 - model.current2,
            length: model.current1,
        };
        model.squares.push(square);
        model.current2 -= model.current1;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);

    draw.rect()
        .x_y(0.0, 0.0)
        .w_h(model.num1 as f32, model.num2 as f32)
        .stroke_weight(1.0)
        .stroke_color(BLACK);

    for square in &model.squares {
        let center_x = square.left as f32 + square.length as f32 / 2.0;
        let center_y = square.bottom as f32 + square.length as f32 / 2.0;
        draw.rect()
            .x_y(
                center_x - model.num1 as f32 / 2.0,
                center_y - model.num2 as f32 / 2.0,
            )
            .w_h(square.length as f32, square.length as f32)
            .color(LIGHTCYAN)
            .stroke_weight(1.0)
            .stroke_color(BLACK);
    }

    draw.to_frame(app, &frame).unwrap();
}
