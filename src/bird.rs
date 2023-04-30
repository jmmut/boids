use macroquad::prelude::Vec2;

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
            front: self.pos + self.dir,
            left: self.pos - self.dir * 0.5 + rotate_left(self.dir * 0.5),
            right: self.pos - self.dir * 0.5 + rotate_right(self.dir * 0.5),
        }
    }

    pub fn rotate(&mut self, angle_in_radians: f32) {
        let new_unnormalized_dir = rotate_angle(self.dir, angle_in_radians);
        self.dir = new_unnormalized_dir.normalize() * self.speed;
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

    fn assert_float_eq(a: f32, b: f32) {
        const EPSILON: f32 = 0.0001;
        if (a - b).abs() > EPSILON {
            panic!("floats were not approximately equal!\n  {}\n  {}", a, b);
        }
    }

    fn assert_vec2_eq(a: Vec2, b: Vec2) {
        assert_float_eq(a.x, b.x);
        assert_float_eq(a.y, b.y);
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
        bird.rotate(PI * 0.5);
        let rotated_front = bird.get_triangle().front;
        let down_left = Vec2::new(-0.1, 0.5);
        assert_vec2_eq(rotated_front, down_left);
    }
}
