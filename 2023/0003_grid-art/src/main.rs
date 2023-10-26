use nannou::color::*;
use nannou::lyon::lyon_tessellation::LineCap;
use nannou::prelude::*;
use nannou::rand::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    start_animation: bool,
    rng: StdRng,
    time: f32,
    grid: [[usize; 10]; 10],
    row: usize,
    col: usize,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(408, 408)
        .key_pressed(key_pressed)
        .view(view)
        .build()
        .unwrap();
    Model {
        _window,
        start_animation: false,
        rng: StdRng::seed_from_u64(57),
        time: 0.0,
        grid: [[0; 10]; 10],
        row: 0,
        col: 0,
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

    if app.time < model.time + 0.1 {
        return;
    }
    model.time = app.time;

    model.row = model.rng.gen_range(0..10);
    model.col = model.rng.gen_range(0..10);
    model.grid[model.row][model.col] += 1;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);

    for row in 0..10 {
        for col in 0..10 {
            let direction = model.grid[row][col] % 2;
            draw.line()
                .start(pt2(
                    -200.0 + row as f32 * 40.0,
                    -200.0 + col as f32 * 40.0 + 40.0 - 40.0 * direction as f32,
                ))
                .end(pt2(
                    -200.0 + row as f32 * 40.0 + 40.0,
                    -200.0 + col as f32 * 40.0 + 40.0 * direction as f32,
                ))
                .color(STEELBLUE)
                .weight(10.0)
                .caps(LineCap::Round);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
