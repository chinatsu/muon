use super::{Menu, Targets, Tetris};
use crate::GameLoop;
use macroquad::prelude::*;
pub struct MainState {
    states: Vec<Box<dyn GameLoop>>,
    current: usize,
}

impl MainState {
    pub fn new() -> MainState {
        MainState {
            states: vec![
                Box::new(Menu::new()),
                Box::new(Tetris::new()),
                Box::new(Targets::new()),
            ],
            current: 0,
        }
    }
}

impl GameLoop for MainState {
    fn handle_inputs(&mut self) {
        if is_key_pressed(KeyCode::Enter) || touches().len() >= 3 {
            self.current = (self.current + 1) % self.states.len();
        }
        self.states[self.current].handle_inputs();
    }
    fn update(&mut self) {
        self.states[self.current].update();
    }
    fn draw(&self) {
        self.states[self.current].draw();
    }
}
