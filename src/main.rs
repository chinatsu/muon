use macroquad::prelude::*;

mod utils;
use utils::{from_hex, random_color};
mod state;
use state::{GameLoop, MainState};
mod status;
use status::Status;

#[macroquad::main("muon")]
async fn main() {
    let bg = from_hex("#222222");
    let mut components: Vec<Box<dyn GameLoop>> =
        vec![Box::new(Status::new()), Box::new(MainState::new())];

    loop {
        components.iter_mut().for_each(|c| c.handle_inputs());
        components.iter_mut().for_each(|c| c.update());
        clear_background(bg);
        components.iter().for_each(|c| c.draw());
        next_frame().await
    }
}
