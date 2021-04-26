use crate::GameLoop;
use macroquad::prelude::*;
pub struct Menu {
    message: String,
    y: f32,
}

impl Menu {
    pub fn new() -> Menu {
        Menu {
            message: String::from("Menu mode!"),
            y: 20.0,
        }
    }
}

impl GameLoop for Menu {
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
