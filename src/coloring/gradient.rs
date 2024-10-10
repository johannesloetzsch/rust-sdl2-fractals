use sdl2::pixels::Color;
use super::helper::*;

/** Colors a number i
 *  Increasing numbers are mapped to:
 *  > black -> red -> green -> blue -> black
 **/
pub fn gradient_rgb(i: i32) -> Color {
    let r = norm_u8(0.0 - i.abs_diff(20) as f32, -10.0, 20.0);
    let g = norm_u8(0.0 - i.abs_diff(30) as f32, -15.0, 15.0);
    let b = norm_u8(0.0 - i.abs_diff(40) as f32, -20.0, 10.0);
    Color::RGB(r, g, b)
}
