use super::GameLoop;
use macroquad::prelude::*;
mod ball;
pub use ball::Ball;

pub struct Targets {
    player: Ball,
    targets: Vec<Ball>,
    score: f32,
    difficulty: f32,
}

impl Targets {
    pub fn new() -> Targets {
        Targets {
            player: Ball::with_color(60.0, Color::new(1.0, 1.0, 1.0, 0.5)),
            targets: Vec::new(),
            score: 0.0,
            difficulty: 60.0,
        }
    }
}

impl GameLoop for Targets {
    fn handle_inputs(&mut self) {
        let (x, y) = mouse_position();
        self.player.update_coords(x, y);
        if is_mouse_button_pressed(MouseButton::Left) {
            self.score += self
                .targets
                .iter()
                .filter(|target| self.player.touches(&target))
                .map(|target| self.player.score(&target))
                .sum::<f32>();
            self.targets = self
                .targets
                .iter()
                .filter(|target| !self.player.touches(&target))
                .map(|target| target.clone())
                .collect();
        }
    }

    fn update(&mut self) {
        if self.targets.is_empty() {
            self.difficulty = (self.difficulty - 0.1).max(1.0);
            self.targets.push(Ball::new(self.difficulty));
        }
    }

    fn draw(&self) {
        self.targets.iter().for_each(|target| target.draw());
        self.player.draw();
        draw_text(
            &format!("Score: {:.2}", self.score),
            screen_width() - 200.0,
            100.0,
            20.0,
            WHITE,
        );
    }
}
