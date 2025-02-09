mod assertions;
mod bird;
mod bots;

use crate::bird::{Bird, BirdTriangle, TARGET_SPEED};
use crate::bots::{control_bot_birds, spawn_birds};
use macroquad::miniquad::date::now;
use macroquad::prelude::*;
use std::f32::consts::PI;

const DEFAULT_WINDOW_TITLE: &'static str = "Boids";
const DEFAULT_WINDOW_WIDTH: i32 = 256 * 4;
const DEFAULT_WINDOW_HEIGHT: i32 = 256 * 3;
const ACCELERATION: f32 = 0.5; // in pixels per frame squared
const ANGULAR_SPEED: f32 = PI * 0.02; // in radians per frame
const BOT_COUNT: usize = 1000;

#[macroquad::main(window_conf)]
async fn main() {
    let screen_center = Vec2::new(screen_width() * 0.5, screen_height() * 0.5);
    let mut player_bird = Bird::new(screen_center, Vec2::new(TARGET_SPEED, 0.0));
    let fovy = 45.0;
    let camera_pos = vec3(
        screen_center.x,
        screen_center.y,
        screen_center.x / (fovy * 0.5 / 360.0 * 2.0 * PI).tan(),
    );
    let mut bot_birds = spawn_default_birds();
    let mut previous_now = now();
    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::R) {
            bot_birds = spawn_default_birds();
        }

        control_player_bird(&mut player_bird);
        control_bot_birds(
            &mut bot_birds,
            &player_bird,
            screen_width(),
            screen_height(),
        );

        clear_background(LIGHTGRAY);
        set_3d_camera(fovy, camera_pos);
        draw_grid(20, 1., BLACK, GRAY);
        draw_cube_wires(vec3(0., 1., -6.), vec3(2., 2., 2.), DARKGREEN);

        draw_bird(&player_bird, DARKPURPLE);
        for bird in &bot_birds {
            draw_bird(bird, DARKGREEN);
        }

        set_default_camera();
        draw_fps(&mut previous_now);
        next_frame().await
    }
}

fn set_3d_camera(fovy: f32, camera_pos: Vec3) {
    set_camera(&Camera3D {
        position: camera_pos,
        up: vec3(0.0, 1.0, 0.0),
        target: camera_pos + vec3(0.0, 0.0, -1.0),
        fovy,
        ..Default::default()
    });
}

fn draw_fps(previous_now: &mut f64) {
    let new_now = now();
    let fps = 1.0 / (new_now - *previous_now);
    draw_text(&format!("FPS: {}", fps), 30.0, 30.0, 16.0, BLACK);
    *previous_now = new_now;
}

fn window_conf() -> Conf {
    Conf {
        window_title: DEFAULT_WINDOW_TITLE.to_owned(),
        window_width: DEFAULT_WINDOW_WIDTH,
        window_height: DEFAULT_WINDOW_HEIGHT,
        ..Default::default()
    }
}

fn spawn_default_birds() -> Vec<Bird> {
    spawn_birds(
        BOT_COUNT,
        Vec2::new(0.0, 0.0),
        Vec2::new(screen_width(), screen_height()),
    )
}

fn control_player_bird(bird: &mut Bird) {
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
    // draw_triangle(front, left, right, color);
    // draw_sphere(vec3(front.x, front.y, 0.0), bird.get_speed(), None, color)
    draw_cube(
        vec3(front.x, front.y, 0.0),
        Vec3::splat(bird.get_speed()),
        None,
        color,
    )
}
