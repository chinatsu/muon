use macroquad::prelude::*;
use std::time::SystemTime;

mod utils;
use utils::{from_hex, random_color};
mod state;
use state::{GameLoop, MainState};
mod status;
use status::Status;

#[macroquad::main("muon")]
async fn main() {
    seed_randomizer();
    let mut components: Vec<Box<dyn GameLoop>> =
        vec![Box::new(MainState::new()), Box::new(Status::new())];

    loop {
        components.iter_mut().for_each(|c| c.handle_inputs());
        components.iter_mut().for_each(|c| c.update());
        clear_background(from_hex("#222222"));
        components.iter().for_each(|c| c.draw());
        next_frame().await
    }
}

fn seed_randomizer() {
    let time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_else(|e| std::panic::panic_any(e));
    let seed: u64 = time.subsec_millis() as u64 + time.subsec_nanos() as u64;
    rand::srand(seed);
}
