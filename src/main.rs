mod assertions;
mod bird;
mod bots;

use crate::bird::{Bird, BirdTriangle, TARGET_SPEED};
use crate::bots::{control_bot_birds, in_modulo_range, in_modulo_range_i, spawn_birds};
use macroquad::miniquad::date::now;
use macroquad::prelude::*;
use std::f32::consts::PI;

const DEFAULT_WINDOW_TITLE: &'static str = "Boids";
const DEFAULT_WINDOW_WIDTH: i32 = 256 * 5;
const DEFAULT_WINDOW_HEIGHT: i32 = 256 * 3;
const ACCELERATION: f32 = 0.5; // in pixels per frame squared
const ANGULAR_SPEED: f32 = PI * 0.02; // in radians per frame
const BOT_COUNT: usize = 1000;

#[macroquad::main(window_conf)]
async fn main() {
    let mut grabbed = false;
    set_cursor_grab(grabbed);
    show_mouse(!grabbed);
    let map_size = vec3(1000.0, 1000.0, 200.0);
    let min_pos = vec3(-map_size.x * 0.5, -map_size.y * 0.5, 40.0);
    let max_pos = min_pos + map_size;
    // let screen_center = Vec2::new(screen_width() * 0.5, screen_height() * 0.5);
    let mut player_bird = Bird::new(vec3(0.0, 0.0, 100.0), vec3(TARGET_SPEED, 0.0, 0.0));
    let fovy = 45.0;
    // let radians = fovy * 0.5 / 360.0 * 2.0 * PI * 1.5; // why the 1.5???
    let mut camera_pos = vec3(
        0.0,
        -map_size.y * 0.5,
        max_pos.z * 5.0,
        // - screen_center.x / radians.tan(),
    );
    let mut camera_dir = vec3(0.0, 1.0, -1.5);
    let up = vec3(0.0, 0.0, 1.0);
    let mut bot_birds = respawn_default_bots(map_size, min_pos, max_pos);
    let mut paused = true;

    let mut last_mouse_position: Vec2 = mouse_position().into();
    let mut previous_now = now();
    let mut last_draw = 0.0;
    let mut previous_fps = 0.0;
    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::LeftAlt) || is_key_pressed(KeyCode::RightAlt) {
            grabbed = !grabbed;
            set_cursor_grab(grabbed);
            show_mouse(!grabbed);
        }
        if is_key_pressed(KeyCode::R) {
            bot_birds = respawn_default_bots(map_size, min_pos, max_pos);
        }
        if is_key_pressed(KeyCode::Space) {
            paused = !paused;
        }
        control_camera(
            &mut last_mouse_position,
            &mut camera_pos,
            &mut camera_dir,
            up,
        );
        if !paused {
            control_player_bird(&mut player_bird, min_pos, max_pos);
            control_bot_birds(
                &mut bot_birds,
                &player_bird,
                min_pos,
                max_pos,
                Some(player_bird.get_pos()),
                // None,
            );
        }
        clear_background(GRAY);
        set_3d_camera(fovy, camera_pos, camera_dir, up);
        draw_grid(map_size, 1., BLACK, DARKGRAY, camera_pos.z);
        draw_cube_wires(vec3(0., 0., 6.), vec3(2., 2., 2.), DARKGREEN);

        draw_bird(&player_bird, WHITE);
        for bird in &bot_birds[0..bot_birds.len() / 2] {
            draw_bird(bird, GREEN);
        }
        for bird in &bot_birds[bot_birds.len() / 2..] {
            draw_bird(bird, YELLOW);
        }

        set_default_camera();
        draw_fps(&mut previous_now, &mut last_draw, &mut previous_fps);
        next_frame().await
    }
}

fn respawn_default_bots(map_size: Vec3, min_pos: Vec3, max_pos: Vec3) -> Vec<Bird> {
    let mut bot_birds = spawn_birds(BOT_COUNT, min_pos, min_pos + map_size * 0.5);
    let mut bot_birds_2 = spawn_birds(BOT_COUNT, min_pos + map_size * 0.5, max_pos);
    bot_birds.append(&mut bot_birds_2);
    bot_birds
}

fn window_conf() -> Conf {
    Conf {
        window_title: DEFAULT_WINDOW_TITLE.to_owned(),
        window_width: DEFAULT_WINDOW_WIDTH,
        window_height: DEFAULT_WINDOW_HEIGHT,
        ..Default::default()
    }
}

fn control_camera(
    last_mouse_position: &mut Vec2,
    camera_pos: &mut Vec3,
    camera_dir: &mut Vec3,
    up: Vec3,
) {
    let camera_rotation_speed = 0.03;
    let look_speed = 0.003;
    let camera_speed = if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
        3.0
    } else {
        0.3
    };
    if is_key_down(KeyCode::A) {
        let left = up.cross(*camera_dir);
        *camera_dir += left * camera_rotation_speed;
        *camera_dir = camera_dir.normalize();
    }
    if is_key_down(KeyCode::D) {
        let left = up.cross(*camera_dir);
        *camera_dir -= left * camera_rotation_speed;
        *camera_dir = camera_dir.normalize();
    }

    let mouse_position: Vec2 = mouse_position().into();
    let mouse_delta = mouse_position - *last_mouse_position;
    *last_mouse_position = mouse_position;

    let left = up.cross(*camera_dir);
    *camera_dir -= left * mouse_delta.x * look_speed;
    *camera_dir = camera_dir.normalize();

    let mut pitch = mouse_delta.y * look_speed;
    pitch = pitch.clamp(-1.5, 1.5);
    *camera_dir -= up * pitch;

    *camera_dir = camera_dir.normalize();

    if is_key_down(KeyCode::A) {
        let left = up.cross(*camera_dir);
        *camera_dir += left * camera_rotation_speed;
        *camera_dir = camera_dir.normalize();
    }
    if is_key_down(KeyCode::D) {
        let left = up.cross(*camera_dir);
        *camera_dir -= left * camera_rotation_speed;
        *camera_dir = camera_dir.normalize();
    }
    if is_key_down(KeyCode::W) {
        *camera_pos += *camera_dir * camera_speed;
    }
    if is_key_down(KeyCode::S) {
        *camera_pos -= *camera_dir * camera_speed;
    }
}

fn control_player_bird(bird: &mut Bird, min_pos: Vec3, max_pos: Vec3) {
    if is_key_down(KeyCode::Left) {
        bird.rotate(ANGULAR_SPEED);
    }
    if is_key_down(KeyCode::Right) {
        bird.rotate(-ANGULAR_SPEED);
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
    bird.advance_toroid(min_pos, max_pos);
}

fn draw_bird(bird: &Bird, color: Color) {
    // let BirdTriangle { front, left, right } = bird.get_triangle();
    // draw_triangle(front, left, right, color);
    // draw_sphere(vec3(front.x, front.y, 0.0), bird.get_speed(), None, color)
    draw_cube(
        bird.get_pos(),
        Vec3::splat(bird.get_speed() * 0.5 + 1.0),
        None,
        color,
    )
}

fn set_3d_camera(fovy: f32, camera_pos: Vec3, camera_dir: Vec3, up: Vec3) {
    set_camera(&Camera3D {
        position: camera_pos,
        up,
        target: camera_pos + camera_dir,
        fovy,
        ..Default::default()
    });
}

fn draw_fps(previous_now: &mut f64, last_draw: &mut f64, previous_fps: &mut f64) {
    let new_now = now();
    let delay_refresh_seconds = 0.1;
    let should_update = new_now - *last_draw > delay_refresh_seconds;
    let fps = if should_update {
        *last_draw = new_now;
        1.0 / (new_now - *previous_now)
    } else {
        *previous_fps
    };
    draw_text(&format!("FPS: {:.2}", fps), 30.0, 30.0, 16.0, BLACK);
    *previous_now = new_now;
    *previous_fps = fps;
}
pub fn draw_grid(
    slices: Vec3,
    spacing: f32,
    axes_color: Color,
    mut other_color: Color,
    distance: f32,
) {
    let half_slices_x = (slices.x as i32) / 2;
    let half_slices_y = (slices.y as i32) / 2;
    let min = 20.0;
    let max = 200.0;
    let distance = (distance.abs().clamp(min, max) - min) / (max - min);
    other_color.a *= 1.0 - distance;
    for i in -half_slices_x..half_slices_x + 1 {
        let color = if in_modulo_range_i(i, 0, 10) == 0 {
            axes_color
        } else {
            other_color
        };

        draw_line_3d(
            vec3(i as f32 * spacing, -half_slices_x as f32 * spacing, 0.),
            vec3(i as f32 * spacing, half_slices_x as f32 * spacing, 0.),
            color,
        );
    }

    for i in -half_slices_y..half_slices_y + 1 {
        let color = if in_modulo_range_i(i, 0, 10) == 0 {
            axes_color
        } else {
            other_color
        };

        draw_line_3d(
            vec3(-half_slices_x as f32 * spacing, i as f32 * spacing, 0.),
            vec3(half_slices_y as f32 * spacing, i as f32 * spacing, 0.),
            color,
        );
    }
}
