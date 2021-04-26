use macroquad::prelude::*;

mod utils;
use utils::{from_hex, random_color};
mod state;
use state::{GameLoop, MainState};
mod status;
use status::Status;

#[macroquad::main("muon")]
async fn main() {
    let mut components: Vec<Box<dyn GameLoop>> =
        vec![Box::new(Status::new()), Box::new(MainState::new())];

    loop {
        components.iter_mut().for_each(|c| c.handle_inputs());
        components.iter_mut().for_each(|c| c.update());
        clear_background(from_hex("#222222"));
        components.iter().for_each(|c| c.draw());
        next_frame().await
    }
}
