use crate::GameLoop;
use macroquad::prelude::*;
pub struct Menu {
    message: String,
    wants_switch: bool,
    y: f32,
}

impl Menu {
    pub fn new() -> Menu {
        Menu {
            message: String::from("Menu mode! Press Enter to cycle modes"),
            y: 20.0,
            wants_switch: false,
        }
    }
}

impl GameLoop for Menu {
    fn handle_inputs(&mut self) {
        if is_key_pressed(KeyCode::Enter) {
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
