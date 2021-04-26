use super::{Menu, Targets, Tetris};
use crate::GameLoop;
use macroquad::prelude::*;
pub struct MainState {
    components: Vec<Box<dyn GameLoop>>,
    index: usize,
}

impl MainState {
    pub fn new() -> MainState {
        MainState {
            components: vec![
                Box::new(Menu::new()),
                Box::new(Tetris::new()),
                Box::new(Targets::new()),
            ],
            index: 0,
        }
    }
}

impl GameLoop for MainState {
    fn handle_inputs(&mut self) {
        self.components[self.index].handle_inputs();
    }
    fn update(&mut self) {
        let index = self.index;
        if self.components[index].wants_switch() {
            self.index = (self.index + 1) % self.components.len();
            self.components[index].reset_switch();
        }
        self.components[self.index].update();
    }
    fn draw(&self) {
        self.components[self.index].draw();
    }

    fn wants_switch(&self) -> bool {
        false
    }

    fn reset_switch(&mut self) {}
}
