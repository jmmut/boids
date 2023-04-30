
mod bird;

use std::f32::consts::PI;
use macroquad::prelude::*;
use macroquad::ui::root_ui;
use macroquad::ui::widgets::Label;
use crate::bird::{Bird, BirdTriangle};

const DEFAULT_WINDOW_TITLE: &'static str = "Boids";
const DEFAULT_WINDOW_WIDTH: i32 = 256*4;
const DEFAULT_WINDOW_HEIGHT: i32 = 256*3;
const ANGULAR_SPEED: f32 = PI * 0.02; // in radians per frame

#[macroquad::main(window_conf)]
async fn main() {
    let mut bird = Bird::new(Vec2::new(screen_width()*0.5, screen_height()*0.5), Vec2::new(10.0, 0.0));
    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        control_bird(&mut bird);
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
fn control_bird(bird: &mut Bird) {
    if is_key_down(KeyCode::Left) {
        bird.rotate(-ANGULAR_SPEED);
    }
    if is_key_down(KeyCode::Right) {
        bird.rotate(ANGULAR_SPEED);
    }
    bird.advance_toroid(screen_width(), screen_height());
}
fn draw_bird(bird: &Bird) {
    let BirdTriangle { front, left, right } = bird.get_triangle();
    draw_triangle(front, left, right, DARKGREEN);
}
