mod bird;

use macroquad::prelude::*;
use macroquad::ui::root_ui;
use macroquad::ui::widgets::Label;
use crate::bird::{Bird, BirdTriangle};

const DEFAULT_WINDOW_TITLE: &'static str = "Boids";
const DEFAULT_WINDOW_WIDTH: i32 = 256*4;
const DEFAULT_WINDOW_HEIGHT: i32 = 256*3;

#[macroquad::main(window_conf)]
async fn main() {
    let bird = Bird::new(Vec2::new(screen_width()*0.5, screen_height()*0.5), Vec2::new(10.0, 0.0));
    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        clear_background(LIGHTGRAY);
        draw_bird(&bird);
        next_frame().await
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: DEFAULT_WINDOW_TITLE.to_owned(),
        window_width: DEFAULT_WINDOW_WIDTH,
        window_height: DEFAULT_WINDOW_HEIGHT,
        ..Default::default()
    }
}

fn draw_bird(bird: &Bird) {
    let BirdTriangle { front, left, right } = bird.get_triangle();
    draw_triangle(front, left, right, DARKGREEN);
}
