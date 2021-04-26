use macroquad::color::{Color, BLACK};
use macroquad::rand::gen_range;

pub fn from_hex(hex: &str) -> Color {
    match hex[1..]
        .as_bytes()
        .chunks(2)
        .map(|x| {
            let val = unsafe { std::str::from_utf8_unchecked(x) };
            u32::from_str_radix(val, 16).unwrap() as f32 / 256.0
        })
        .collect::<Vec<f32>>()[..]
    {
        [r, g, b, a] => Color::new(r, g, b, a),
        [r, g, b] => Color::new(r, g, b, 1.0),
        _ => BLACK,
    }
}

pub fn random_color() -> Color {
    let r = gen_range::<u8>(0, 255);
    let g = gen_range::<u8>(0, 255);
    let b = gen_range::<u8>(0, 255);
    Color::from_rgba(r, g, b, 255)
}
