use crate::bird::{Bird, SIGHT_DISTANCE, TARGET_SPEED};
use macroquad::math::{vec3, Vec3};
use macroquad::prelude::Vec2;
use std::f32::consts::PI;

const BOT_DEFAULT_SPEED: f32 = TARGET_SPEED;
const PEER_PRESSURE_FACTOR: f32 = 0.1; // in world units per frame squared
const PERSONAL_SPACE: f32 = SIGHT_DISTANCE * 0.5; // in world units
const PERSONAL_SPACE_SQUARED: f32 = PERSONAL_SPACE * PERSONAL_SPACE; // in world units
const PERSONAL_SPACE_STRENGTH: f32 = 0.2; // [0, 1] coefficient
const COHESION_FACTOR: f32 = 0.01;
const MAP_LIMIT_CORRECTION: f32 = 1.0;
const TARGET_ATTRACTION: f32 = 0.2;

pub fn spawn_birds(count: usize, min_pos: Vec3, max_pos: Vec3) -> Vec<Bird> {
    let mut seed = 3453457.0;
    let mut bots = Vec::with_capacity(count);
    let mut rnd = || iterate_hash(&mut seed);
    for _ in 0..count {
        bots.push(Bird::new(
            Vec3::new(
                in_modulo_range(rnd(), min_pos.x, max_pos.x),
                in_modulo_range(rnd(), min_pos.y, max_pos.y),
                in_modulo_range(rnd(), min_pos.z, max_pos.z),
            ),
            angle_to_coords3(
                in_modulo_range(rnd(), 0.0, 2.0 * PI),
                in_modulo_range(rnd(), -1.0, 1.0),
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

/// See tests for exact behaviour. assumes min < max
pub fn in_modulo_range(value: f32, min: f32, max: f32) -> f32 {
    assert!(min < max, "{} < {}", min, max);
    if value < min {
        let diff = min - value;
        let range = max - min;
        let base = min - (diff / range).ceil() * range;
        return (value - base) % range + min;
    }
    (value - min) % (max - min) + min
}
pub fn in_modulo_range_i(value: i32, min: i32, max: i32) -> i32 {
    assert!(min < max, "{} < {}", min, max);
    if value < min {
        let diff = min - value;
        let range = max - min;
        let base = min - diff / range * range;
        return (value - base) % range + min;
    }
    (value - min) % (max - min) + min
}

fn angle_to_coords(angle: f32) -> Vec2 {
    Vec2::new(angle.cos(), angle.sin())
}
fn angle_to_coords3(angle: f32, pitch: f32) -> Vec3 {
    Vec3::new(angle.cos(), angle.sin(), pitch)
}

pub fn control_bot_birds(
    bot_birds: &mut Vec<Bird>,
    player_bird: &Bird,
    min_pos: Vec3,
    max_pos: Vec3,
    target: Option<Vec3>,
) {
    for i_current_bird in 0..bot_birds.len() {
        bot_birds
            .get_mut(i_current_bird)
            .unwrap()
            .advance_toroid(min_pos, max_pos);
        let current_bird = bot_birds.get(i_current_bird).unwrap();

        let height_limits = correct_map_limit(min_pos, max_pos, current_bird);
        let target = correct_for_target(current_bird.get_pos(), target);
        let mut other_birds_direction = Vec3::default();
        let mut other_birds_count = 0;
        let mut position_accumulator = PositionAccumulator::new();
        let current_bird1 = bot_birds.get(i_current_bird).unwrap();
        if current_bird1.can_see(player_bird) {
            other_birds_count += 1;
            other_birds_direction += player_bird.get_direction();
            position_accumulator.add_position(player_bird.get_pos());
        }
        let mut closest_bird_pos = player_bird.get_pos();
        let mut closest_bird_distance_squared = player_bird.squared_distance_to(current_bird);

        for i_other_bird in 0..bot_birds.len() {
            if i_other_bird != i_current_bird {
                let other_bird = bot_birds.get(i_other_bird).unwrap();
                if current_bird.can_see(other_bird) {
                    other_birds_count += 1;
                    other_birds_direction += other_bird.get_direction();
                    position_accumulator.add_position(other_bird.get_pos());
                }
                let distance = current_bird.squared_distance_to(other_bird);
                if distance < closest_bird_distance_squared {
                    closest_bird_distance_squared = distance;
                    closest_bird_pos = other_bird.get_pos();
                }
            }
        }
        let alignment = if other_birds_count == 0 {
            Vec3::default()
        } else {
            other_birds_direction / other_birds_count as f32
        };
        if alignment.is_nan() {
            println!("alignment");
        }
        let separation = if closest_bird_distance_squared < PERSONAL_SPACE_SQUARED {
            -(closest_bird_pos - current_bird.get_pos()) * PERSONAL_SPACE_STRENGTH
        } else {
            Vec3::default()
        };
        let cohesion =
            position_accumulator.get_average_from(current_bird.get_pos()) * COHESION_FACTOR;
        // let cohesion = Vec3::default();
        let direction_modifier = alignment + separation + cohesion + height_limits + target;
        if direction_modifier.is_nan() {
            // panic!("should not happen, put breakpoint here");
        }
        bot_birds
            .get_mut(i_current_bird)
            .unwrap()
            .modify_direction(direction_modifier, PEER_PRESSURE_FACTOR);
    }
}

fn correct_for_target(pos: Vec3, target: Option<Vec3>) -> Vec3 {
    if let Some(target) = target {
        let diff = target - pos;
        diff.normalize() * TARGET_ATTRACTION
    } else {
        Vec3::default()
    }
}

fn correct_map_limit(min_pos: Vec3, max_pos: Vec3, current_bird: &Bird) -> Vec3 {
    let height_limits =
        if current_bird.get_pos().z < min_pos.z && current_bird.get_direction().z < 0.0 {
            vec3(0.0, 0.0, MAP_LIMIT_CORRECTION)
        } else if current_bird.get_pos().z > max_pos.z && current_bird.get_direction().z > 0.0 {
            vec3(0.0, 0.0, -MAP_LIMIT_CORRECTION)
        } else {
            Vec3::default()
        };
    height_limits
}

struct PositionAccumulator {
    added_positions: Vec3,
    position_count: i32,
}

impl PositionAccumulator {
    pub fn new() -> Self {
        Self {
            added_positions: Vec3::default(),
            position_count: 0,
        }
    }
    pub fn add_position(&mut self, other_pos: Vec3) {
        self.added_positions += other_pos;
        self.position_count += 1;
    }
    pub fn get_average_from(&self, reference: Vec3) -> Vec3 {
        if self.position_count == 0 {
            reference
        } else {
            self.added_positions / self.position_count as f32 - reference
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assertions::{assert_float_eq, assert_vec2_eq};

    #[test]
    fn test_spawn_birds() {
        let min_pos = Vec3::new(40.0, 30.0, 20.0);
        let max_pos = Vec3::new(400.0, 300.0, 200.0);
        let bots = spawn_birds(10, min_pos, max_pos);
        for bot in bots {
            assert!(bot.get_pos().x >= min_pos.x && bot.get_pos().x <= max_pos.x);
            assert!(bot.get_pos().y >= min_pos.y && bot.get_pos().y <= max_pos.y);
            assert!(bot.get_pos().z >= min_pos.z && bot.get_pos().z <= max_pos.z);
            assert_float_eq(bot.get_speed(), BOT_DEFAULT_SPEED);
        }
    }

    #[test]
    fn test_float_modulo() {
        assert_eq!(107.0 % 100.0, 7.0);
        assert_eq!(-107.0 % 100.0, -7.0);
    }

    #[test]
    fn test_in_modulo_range() {
        assert_float_eq(in_modulo_range(3.0, 0.0, 10.0), 3.0);
        assert_float_eq(in_modulo_range(13.0, 0.0, 10.0), 3.0);
        assert_float_eq(in_modulo_range(13.0, 20.0, 30.0), 23.0);
        assert_float_eq(in_modulo_range(13.0, 50.0, 60.0), 53.0);
        assert_float_eq(in_modulo_range(-7.0, 0.0, 10.0), 3.0);
        assert_float_eq(in_modulo_range(-27.0, 0.0, 10.0), 3.0);
        assert_float_eq(in_modulo_range(-27.0, 20.0, 30.0), 23.0);
        assert_float_eq(in_modulo_range(-57.0, -30.0, -20.0), -27.0);
        assert_float_eq(in_modulo_range(13.0, -30.0, -20.0), -27.0);
    }

    #[test]
    fn test_angle_to_coords() {
        assert_vec2_eq(angle_to_coords(0.0), Vec2::new(1.0, 0.0));
        assert_vec2_eq(angle_to_coords(PI * 0.5), Vec2::new(0.0, 1.0));
        assert_vec2_eq(angle_to_coords(PI), Vec2::new(-1.0, 0.0));
    }
}
