mod assertions;
mod bird;
mod bots;

use crate::bird::{Bird, BirdTriangle};
use crate::bots::spawn_birds;
use macroquad::prelude::*;
use std::f32::consts::PI;

const DEFAULT_WINDOW_TITLE: &'static str = "Boids";
const DEFAULT_WINDOW_WIDTH: i32 = 256 * 4;
const DEFAULT_WINDOW_HEIGHT: i32 = 256 * 3;
const ANGULAR_SPEED: f32 = PI * 0.02; // in radians per frame
const ACCELERATION: f32 = 0.5; // in pixels per frame squared

#[macroquad::main(window_conf)]
async fn main() {
    let mut player_bird = Bird::new(
        Vec2::new(screen_width() * 0.5, screen_height() * 0.5),
        Vec2::new(10.0, 0.0),
    );
    let mut bot_birds = spawn_birds(
        100,
        Vec2::new(0.0, 0.0),
        Vec2::new(screen_width(), screen_height()),
    );
    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        control_bird(&mut player_bird);
        clear_background(LIGHTGRAY);
        draw_bird(&player_bird, DARKPURPLE);
        for bird in &mut bot_birds {
            bird.advance_toroid(screen_width(), screen_height());
            draw_bird(&bird, DARKGREEN);
        }
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
    if is_key_down(KeyCode::Up) {
        bird.modify_speed(ACCELERATION);
    }
    if is_key_down(KeyCode::Down) {
        bird.modify_speed(-ACCELERATION);
    }
    if is_key_pressed(KeyCode::F1) {
        println!("bird: {:?}", bird);
    }
    bird.advance_toroid(screen_width(), screen_height());
}

fn draw_bird(bird: &Bird, color: Color) {
    let BirdTriangle { front, left, right } = bird.get_triangle();
    draw_triangle(front, left, right, color);
}
