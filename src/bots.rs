use crate::bird::{Bird, SIGHT_DISTANCE, TARGET_SPEED};
use macroquad::prelude::Vec2;
use std::f32::consts::PI;

const BOT_DEFAULT_SPEED: f32 = TARGET_SPEED;
const PEER_PRESSURE_FACTOR: f32 = 0.3; // in pixels per frame squared
const PERSONAL_SPACE: f32 = SIGHT_DISTANCE * 0.5; // in pixels
const PERSONAL_SPACE_SQUARED: f32 = PERSONAL_SPACE * PERSONAL_SPACE; // in pixels
const PERSONAL_SPACE_STRENGTH: f32 = 0.2; // [0, 1] coefficient

pub fn spawn_birds(count: usize, min_pos: Vec2, max_pos: Vec2) -> Vec<Bird> {
    let mut seed = 3453457.0;
    let mut bots = Vec::with_capacity(count);
    let mut rnd = || iterate_hash(&mut seed);
    for _ in 0..count {
        bots.push(Bird::new(
            Vec2::new(
                in_modulo_range(rnd(), min_pos.x, max_pos.x),
                in_modulo_range(rnd(), min_pos.y, max_pos.y),
            ),
            angle_to_coords(in_modulo_range(rnd(), 0.0, 2.0 * PI)),
        ));
        bots.last_mut().unwrap().set_speed(BOT_DEFAULT_SPEED);
    }
    bots
}

fn iterate_hash(h: &mut f64) -> f32 {
    *h = (*h * 1.25 + 14351.0) % 16935.0;
    *h as f32
}

fn in_modulo_range(value: f32, min: f32, max: f32) -> f32 {
    (value - min) % (max - min) + min
}

fn angle_to_coords(angle: f32) -> Vec2 {
    Vec2::new(angle.cos(), angle.sin())
}

pub fn control_bot_birds(bot_birds: &mut Vec<Bird>, player_bird: &Bird, map_width: f32, map_height: f32) {
    for i_current_bird in 0..bot_birds.len() {
        bot_birds
            .get_mut(i_current_bird)
            .unwrap()
            .advance_toroid(map_width, map_height);
        let current_bird = bot_birds.get(i_current_bird).unwrap();
        let mut other_birds_direction = Vec2::default();
        let mut other_birds_count = 0;
        accumulate_directions(
            bot_birds.get(i_current_bird).unwrap(),
            player_bird,
            &mut other_birds_direction,
            &mut other_birds_count,
        );
        let mut closest_bird_pos = player_bird.get_pos();
        let mut closest_bird_distance = player_bird.squared_distance_to(current_bird);

        for i_other_bird in 0..bot_birds.len() {
            if i_other_bird != i_current_bird {
                let other_bird = bot_birds.get(i_other_bird).unwrap();
                accumulate_directions(
                    current_bird,
                    other_bird,
                    &mut other_birds_direction,
                    &mut other_birds_count,
                );
                let distance = current_bird.squared_distance_to(other_bird);
                if distance < closest_bird_distance {
                    closest_bird_distance = distance;
                    closest_bird_pos = other_bird.get_pos();
                }
            }
        }
        let mut direction_modifier = other_birds_direction / other_birds_count as f32;
        if closest_bird_distance < PERSONAL_SPACE_SQUARED {
            direction_modifier -=
                (closest_bird_pos - current_bird.get_pos()) * PERSONAL_SPACE_STRENGTH;
        }
        bot_birds
            .get_mut(i_current_bird)
            .unwrap()
            .modify_direction(direction_modifier, PEER_PRESSURE_FACTOR);
    }
}

fn accumulate_directions(
    current_bird: &Bird,
    other_bird: &Bird,
    other_birds_direction: &mut Vec2,
    other_birds_count: &mut i32,
) {
    if current_bird.can_see(other_bird) {
        *other_birds_count += 1;
        *other_birds_direction += other_bird.get_direction();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assertions::{assert_float_eq, assert_vec2_eq};

    #[test]
    fn test_spawn_birds() {
        let min_pos = Vec2::new(40.0, 30.0);
        let max_pos = Vec2::new(400.0, 300.0);
        let bots = spawn_birds(10, min_pos, max_pos);
        for bot in bots {
            assert!(bot.get_pos().x >= min_pos.x && bot.get_pos().x <= max_pos.x);
            assert!(bot.get_pos().y >= min_pos.y && bot.get_pos().y <= max_pos.y);
            assert_float_eq(bot.get_speed(), BOT_DEFAULT_SPEED);
        }
    }

    #[test]
    fn test_float_modulo() {
        assert_eq!(107.0 % 100.0, 7.0);
        assert_eq!(-107.0 % 100.0, -7.0);
    }

    #[test]
    fn test_angle_to_coords() {
        assert_vec2_eq(angle_to_coords(0.0), Vec2::new(1.0, 0.0));
        assert_vec2_eq(angle_to_coords(PI * 0.5), Vec2::new(0.0, 1.0));
        assert_vec2_eq(angle_to_coords(PI), Vec2::new(-1.0, 0.0));
    }
}
