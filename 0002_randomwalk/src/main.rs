use nannou::color::*;
use nannou::prelude::*;
use nannou::rand::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    start_animation: bool,
    rng: StdRng,
    bg_color: Srgb<u8>,
    fg_color: Hsl,
    step_length: f32,
    start: Point2,
    end: Point2,
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
        rng: StdRng::seed_from_u64(53),
        bg_color: WHITE,
        fg_color: hsl(0.0, 0.5, 0.5),
        step_length: 30.0,
        start: pt2(0.0, 0.0),
        end: pt2(0.0, 0.0),
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

    // 前回の終点を始点にする
    model.start = model.end;

    // 一歩の移動増分を求める
    let angle = model.rng.gen_range(0.0..2.0 * PI);
    let vec = vec2(angle.cos(), angle.sin()) * model.step_length;

    // 一歩進んだ先の点を求める
    model.end = model.start + vec;

    // 画面の端に到達したら、逆方向に移動する
    let win = app.window_rect();
    if model.end.x < win.left() || model.end.x > win.right() {
        model.end.x = model.start.x - vec.x;
    }
    if model.end.y < win.bottom() || model.end.y > win.top() {
        model.end.y = model.start.y - vec.y;
    }

    // 角度から色相環を使って色を決める
    model.fg_color = hsl(angle as f32 / (2.0 * PI), 0.5, 0.5);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    if app.elapsed_frames() == 0 {
        draw.background().color(model.bg_color);
    }

    if !model.start_animation {
        draw.to_frame(app, &frame).unwrap();
        return;
    }

    draw.line()
        .color(model.fg_color)
        .start(model.start)
        .end(model.end);

    draw.to_frame(app, &frame).unwrap();
}
