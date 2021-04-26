use crate::GameLoop;
use macroquad::prelude::*;
pub struct Tetris {
    message: String,
    y: f32,
    wants_switch: bool,
}

impl Tetris {
    pub fn new() -> Tetris {
        Tetris {
            message: String::from("Some other mode!"),
            y: 20.0,
            wants_switch: false,
        }
    }
}

impl GameLoop for Tetris {
    fn handle_inputs(&mut self) {
        if is_key_pressed(KeyCode::Q) {
            self.wants_switch = true;
        }
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

    fn wants_switch(&self) -> bool {
        self.wants_switch
    }

    fn reset_switch(&mut self) {
        self.wants_switch = false;
    }
}
