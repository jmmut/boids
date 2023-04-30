use macroquad::prelude::Vec2;

pub const SIGHT_DISTANCE: f32 = 25.0;
const SIGHT_DISTANCE_SQUARED: f32 = SIGHT_DISTANCE * SIGHT_DISTANCE;
const MINIMUM_SPEED: f32 = 1.5; // in pixels per frame
pub const TARGET_SPEED: f32 = 3.0; // in pixels per frame

#[derive(Debug)]
pub struct Bird {
    pos: Vec2,
    dir: Vec2,
    speed: f32,
}

pub struct BirdTriangle {
    pub front: Vec2,
    pub left: Vec2,
    pub right: Vec2,
}

impl Bird {
    pub fn new(pos: Vec2, dir: Vec2) -> Self {
        Self {
            pos,
            dir,
            speed: dir.dot(dir).sqrt(),
        }
    }

    pub fn get_triangle(&self) -> BirdTriangle {
        BirdTriangle {
            front: self.pos + self.dir * 2.0,
            left: self.pos - self.dir + rotate_left(self.dir),
            right: self.pos - self.dir + rotate_right(self.dir),
        }
    }

    pub fn get_speed(&self) -> f32 {
        self.speed
    }

    pub fn get_pos(&self) -> Vec2 {
        self.pos
    }

    pub fn get_direction(&self) -> Vec2 {
        self.dir
    }

    pub fn can_see(&self, other: &Bird) -> bool {
        self.squared_distance_to(other) < SIGHT_DISTANCE_SQUARED
    }

    pub fn squared_distance_to(&self, other: &Bird) -> f32 {
        let diff = self.pos - other.pos;
        diff.dot(diff)
    }

    pub fn rotate(&mut self, angle_in_radians: f32) {
        self.dir = rotate_angle(self.dir, angle_in_radians);
        self.update_dir_magnitude();
    }

    pub fn modify_speed(&mut self, acceleration: f32) {
        if self.speed + acceleration > MINIMUM_SPEED {
            self.speed += acceleration;
            self.update_dir_magnitude();
        }
    }
    pub fn modify_direction(&mut self, acceleration: Vec2, weight: f32) {
        let new_dir = self.dir * (1.0 - weight) + acceleration * weight;
        let new_speed = new_dir.length();
        if new_speed > MINIMUM_SPEED {
            self.dir = new_dir;
            self.speed = new_speed;
        }
    }

    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed;
        self.update_dir_magnitude();
    }

    fn update_dir_magnitude(&mut self) {
        self.dir = self.dir.normalize() * self.speed;
    }

    pub fn advance_toroid(&mut self, width: f32, height: f32) {
        self.pos += self.dir;
        // assumes that increments are smaller than 1 whole screen
        if self.pos.x < 0.0 {
            self.pos.x += width;
        }
        if self.pos.x >= width {
            self.pos.x -= width;
        }
        if self.pos.y < 0.0 {
            self.pos.y += height;
        }
        if self.pos.y >= height {
            self.pos.y -= height;
        }
        self.speed += (TARGET_SPEED - self.speed) * 0.01;
        self.update_dir_magnitude();
    }
}

fn rotate_left(direction: Vec2) -> Vec2 {
    Vec2::new(direction.y, -direction.x)
}
fn rotate_right(direction: Vec2) -> Vec2 {
    Vec2::new(-direction.y, direction.x)
}

// google rotation matrices to understand this
fn rotate_angle(direction: Vec2, angle_in_radians: f32) -> Vec2 {
    Vec2::new(
        angle_in_radians.cos() * direction.x - angle_in_radians.sin() * direction.y,
        angle_in_radians.sin() * direction.x + angle_in_radians.cos() * direction.y,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assertions::assert_vec2_eq;
    use std::f32::consts::PI;

    #[test]
    fn test_rotate_left() {
        let right_down = Vec2::new(0.5, 0.1);
        let down_left = Vec2::new(-0.1, 0.5);
        assert_eq!(rotate_left(down_left), right_down);
    }

    #[test]
    fn test_rotate_right() {
        let right_down = Vec2::new(0.5, 0.1);
        let down_left = Vec2::new(-0.1, 0.5);
        assert_eq!(rotate_right(right_down), down_left);
    }

    #[test]
    fn test_rotate_angle() {
        let right_down = Vec2::new(0.5, 0.1);
        let down_left = Vec2::new(-0.1, 0.5);
        let rotated = rotate_angle(right_down, PI * 0.5);
        assert_vec2_eq(rotated, down_left);
        let rotated = rotate_angle(right_down, PI * 0.4);
        assert_vec2_eq(rotated, Vec2::new(0.0594, 0.50643));
    }

    #[test]
    fn test_rotate_bird() {
        let mut bird = Bird::new(Vec2::default(), Vec2::new(0.5, 0.1));
        let front = bird.get_triangle().front;
        let radians = PI * 0.5;
        bird.rotate(radians);
        let rotated_front = bird.get_triangle().front;
        let expected_rotated = rotate_angle(front, radians);
        assert_vec2_eq(rotated_front, expected_rotated);
    }
}
