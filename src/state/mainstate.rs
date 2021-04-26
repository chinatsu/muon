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
        self.states[self.current].handle_inputs();
    }
    fn update(&mut self) {
        let current = self.current;
        if self.states[current].wants_switch() {
            self.current = (self.current + 1) % self.states.len();
            self.states[current].reset_switch();
        }
        self.states[self.current].update();
    }
    fn draw(&self) {
        self.states[self.current].draw();
    }

    fn wants_switch(&self) -> bool {
        false
    }

    fn reset_switch(&mut self) {}
}
