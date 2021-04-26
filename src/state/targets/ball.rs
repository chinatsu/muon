use crate::random_color;
use macroquad::color::Color;
use macroquad::prelude::{draw_circle, screen_height, screen_width};
use macroquad::rand::rand;

#[derive(Clone, Copy)]
pub struct Ball {
    x: f32,
    y: f32,
    radius: f32,
    color: Color,
}

impl Ball {
    pub fn new(radius: f32) -> Ball {
        Ball {
            x: (rand() as f32 + radius) % screen_width(),
            y: (rand() as f32 + radius) % screen_height(),
            radius: radius,
            color: random_color(),
        }
    }

    pub fn with_color(radius: f32, color: Color) -> Ball {
        Ball {
            x: (rand() as f32 + radius) % screen_width(),
            y: (rand() as f32 + radius) % screen_height(),
            radius: radius,
            color: color,
        }
    }

    pub fn draw(&self) {
        draw_circle(self.x, self.y, self.radius, self.color);
    }

    pub fn update_coords(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    pub fn touches(&self, other: &Ball) -> bool {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        let radii = other.radius + self.radius;
        dx * dx + dy * dy < radii * radii
    }

    pub fn score(&self, other: &Ball) -> f32 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        1.0 / ((dy + dx).abs() + 1.0) * 100.0
    }
}
