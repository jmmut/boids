use macroquad::prelude::Vec2;

pub fn assert_float_eq(a: f32, b: f32) {
    const EPSILON: f32 = 0.0001;
    if (a - b).abs() > EPSILON {
        panic!("floats were not approximately equal!\n  {}\n  {}", a, b);
    }
}

pub fn assert_vec2_eq(a: Vec2, b: Vec2) {
    assert_float_eq(a.x, b.x);
    assert_float_eq(a.y, b.y);
}
