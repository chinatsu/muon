use macroquad::prelude::*;
use macroquad::miniquad::date;

mod color;
use color::{from_hex, random_color};
mod game;
use game::Ball;

#[macroquad::main("muon")]
async fn main() {
    rand::srand(date::now() as u64);
    
    let bg = from_hex("#222222");
    let mut difficulty: f32 = 60.0;

    let mut cursor = game::Ball::with_color(60.0, Color::new(1.0, 1.0, 1.0, 0.5));
    let mut targets: Vec<Ball> = vec![];
    let mut score: f32 = 0.0;
    loop {
        let (x, y) = mouse_position();
        cursor.update_coords(x, y);

        if is_mouse_button_pressed(MouseButton::Left) {
            score += targets.iter().filter(|target| cursor.touches(&target)).map(|target| cursor.score(&target)).sum::<f32>();
            targets = targets.into_iter().filter(|target| !cursor.touches(&target)).collect();
        }

        if targets.is_empty() {
            difficulty = (difficulty - 0.1).max(1.0);
            targets.push(Ball::new(difficulty));
        }

        clear_background(bg);
        targets.iter().for_each(|target| target.draw());
        cursor.draw();
        draw_text(&format!("FPS: {}", get_fps()), screen_width() - 200.0, 20.0, 20.0, WHITE);
        draw_text(&format!("Score: {:.2}", score), screen_width() - 200.0, 40.0, 20.0, WHITE);
        draw_text("Version: 0.1.0", screen_width() - 200.0, 60.0, 20.0, WHITE);
        next_frame().await
        
    }
}