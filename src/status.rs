use crate::GameLoop;
use macroquad::prelude::*;

pub struct Status {
    x: f32,
    y: f32,
}

impl Status {
    pub fn new() -> Status {
        Status { x: 0.0, y: 0.0 }
    }
}

impl GameLoop for Status {
    fn handle_inputs(&mut self) {
        let (x, y) = mouse_position();
        self.x = x;
        self.y = y;
    }

    fn update(&mut self) {}

    fn draw(&self) {
        draw_text(
            &format!("FPS: {}", get_fps()),
            screen_width() - 200.0,
            20.0,
            20.0,
            WHITE,
        );
        draw_text(
            &format!("Mouse: {:.2}, {:.2}", self.x, self.y),
            screen_width() - 200.0,
            40.0,
            20.0,
            WHITE,
        );
        draw_text("Version: 0.1.2", screen_width() - 200.0, 60.0, 20.0, WHITE);
    }

    fn wants_switch(&self) -> bool {
        false
    }

    fn reset_switch(&mut self) {}
}
