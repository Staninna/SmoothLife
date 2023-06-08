use nannou::prelude::*;

mod smooth_life;

fn main() {
    // Window size
    let width = smooth_life::WIDTH as u32;
    let height = smooth_life::HEIGHT as u32;

    // Create the window
    nannou::app(model)
        .update(update)
        .event(event)
        .simple_window(view)
        .size(width, height)
        .run();
}

struct Model {
    grid: smooth_life::Grid,
}

fn model(_app: &App) -> Model {
    Model {
        grid: smooth_life::rand_grid(),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let grid_diff = smooth_life::compute_grid_diff(&model.grid);
    smooth_life::update_grid(&mut model.grid, &grid_diff);
}

fn event(_app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent {
            simple: Some(event),
            ..
        } => match event {
            KeyPressed(key) => match key {
                Key::Space => {
                    model.grid = smooth_life::rand_grid();
                }
                _ => (),
            },
            _ => (),
        },
        _ => (),
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    for y in 0..smooth_life::HEIGHT {
        for x in 0..smooth_life::WIDTH {
            let color = smooth_life::get_color(model.grid[y][x]);
            draw.rect()
                .x_y(
                    x as f32 - smooth_life::WIDTH as f32 / 2.0,
                    y as f32 - smooth_life::HEIGHT as f32 / 2.0,
                )
                .w_h(1.0, 1.0)
                .color(color);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
