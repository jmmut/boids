
use macroquad::prelude::*;
use macroquad::ui::root_ui;
use macroquad::ui::widgets::Label;

const DEFAULT_WINDOW_TITLE: &'static str = "Boids";
const DEFAULT_WINDOW_WIDTH: i32 = 256*4;
const DEFAULT_WINDOW_HEIGHT: i32 = 256*3;

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        clear_background(LIGHTGRAY);
        Label::new("hello macroquad").ui(&mut root_ui());
        next_frame().await
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: DEFAULT_WINDOW_TITLE.to_owned(),
        window_width: DEFAULT_WINDOW_WIDTH,
        window_height: DEFAULT_WINDOW_HEIGHT,
        ..Default::default()
    }
}
