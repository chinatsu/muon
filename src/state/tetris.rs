use crate::GameLoop;
use macroquad::prelude::*;
pub struct Tetris {
    message: String,
    y: f32,
}

impl Tetris {
    pub fn new() -> Tetris {
        Tetris {
            message: String::from("Some other mode!"),
            y: 20.0,
        }
    }
}

impl GameLoop for Tetris {
    fn handle_inputs(&mut self) {
        if is_key_pressed(KeyCode::Up) {
            self.y -= 20.0;
        }
    }

    fn update(&mut self) {
        self.y = (self.y + 1.0) % screen_height();
    }

    fn draw(&self) {
        draw_text(&self.message, 200.0, self.y, 20.0, WHITE);
    }
}
