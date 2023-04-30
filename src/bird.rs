use macroquad::prelude::Vec2;

pub struct Bird {
    pos: Vec2,
    dir: Vec2,
}

pub struct BirdTriangle {
    pub front: Vec2,
    pub left: Vec2,
    pub right: Vec2,
}

impl Bird {
    pub fn new(pos: Vec2, dir: Vec2) -> Self {
        Self { pos, dir }
    }
    pub fn get_triangle(&self) -> BirdTriangle {
        BirdTriangle {
            front: self.pos + self.dir,
            left: self.pos - self.dir*0.5 + rotate_left(self.dir*0.5),
            right: self.pos - self.dir*0.5 + rotate_right(self.dir*0.5),
        }
    }
}

fn rotate_left(direction: Vec2) -> Vec2 {
    Vec2::new(direction.y, -direction.x)
}
fn rotate_right(direction: Vec2) -> Vec2 {
    Vec2::new(-direction.y, direction.x)
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
