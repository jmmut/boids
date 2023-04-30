use crate::bird::Bird;
use macroquad::prelude::Vec2;

const BOT_DEFAULT_SPEED: f32 = 8.0;

pub fn spawn_birds(count: usize, min_pos: Vec2, max_pos: Vec2) -> Vec<Bird> {
    let mut seed = 3453457.0;
    let mut bots = Vec::with_capacity(count);
    for _ in 0..count {
        bots.push(Bird::new(
            Vec2::new(
                in_modulo_range(iterate_hash(&mut seed), min_pos.x, max_pos.x),
                in_modulo_range(iterate_hash(&mut seed), min_pos.y, max_pos.y),
            ),
            Vec2::new(
                in_modulo_range(iterate_hash(&mut seed), 3.0, 5.0),
                in_modulo_range(iterate_hash(&mut seed), 3.0, 5.0),
            ),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assertions::assert_float_eq;

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
}
