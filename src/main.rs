#![allow(dead_code)]

mod benchmark;
mod computer;
mod game;

use game::Game;
use nannou::prelude::*;
use rand::{rngs::ThreadRng, Rng};

struct Model {
    game: Game,
    mouse_clicked: bool,
    rng: ThreadRng,
}

fn main() {
    benchmark::run_tests(3, 1);
    // nannou::app(model)
    //     .event(event)
    //     .simple_window(view)
    //     .size(900, 775)
    //     .run();
}

fn model(_app: &App) -> Model {
    Model {
        game: Game::new(),
        mouse_clicked: false,
        rng: rand::thread_rng(),
    }
}

fn event(app: &App, model: &mut Model, _event: Event) {
    if model.game.is_computer_turn() {
        model.game.make_move(model.rng.gen_range(0..7));
        println!("{:?}", model.game);
    }

    let mouse_click = app.mouse.buttons.left().is_down();
    if !model.mouse_clicked && mouse_click {
        model.game.handle_click(&app.mouse);
    }
    model.mouse_clicked = mouse_click;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    model.game.draw(&draw, &app.mouse);

    draw.to_frame(app, &frame).unwrap();
}
