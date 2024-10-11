use std::f32::consts::PI;

use num::complex::Complex32;
use sdl2::pixels::Color;
use super::helper::*;
use super::hsl::hsl_to_cartesian;

/** Colors complex number z
 *  - real part of z will be colored blueish
 *  - imaginary part of z will be colored greenish
 *  - positive real and imaginary parts are distinguished from their negative counterpart by an
 *  additional red component
 *  - the lightness increases from origin (black) till a distance of radius
 **/
pub fn domain_coloring(z: Complex32, radius: f32, s: f32) -> Color {
    let (r, theta) = z.to_polar();

    let h = (theta+PI)/(2.0*PI);
    let l = 0.3 + 0.3*norm(r, 0.0, radius);

    hsl_to_cartesian(h, s, l)
}
