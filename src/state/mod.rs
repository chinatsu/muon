mod mainstate;
mod menu;
mod targets;
mod tetris;
pub use mainstate::MainState;
pub use menu::Menu;
pub use targets::Targets;
pub use tetris::Tetris;

pub trait GameLoop {
    fn handle_inputs(&mut self);
    fn update(&mut self);
    fn draw(&self);
}
