use crate::bird::Bird;
use macroquad::prelude::Vec2;
use std::f32::consts::PI;

const BOT_DEFAULT_SPEED: f32 = 8.0;

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
