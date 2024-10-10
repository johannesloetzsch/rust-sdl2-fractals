use num::complex::{Complex, Complex32, ComplexFloat};
use sdl2::pixels::Color;
use super::helper::*;

/** Colors complex number z
 *  - real part of z will be colored blueish
 *  - imaginary part of z will be colored greenish
 *  - the lightness increases from origin (black) till a distance of 2*radius (white)
 *  - positive real and imaginary parts are distinguished from their negative counterpart by an
 *  additional red component
 **/
pub fn domain_coloring(z: Complex32, radius: f32) -> Color {
    let re = norm_u8(z.re.abs(), 0.0, radius);
    let im = norm_u8(z.im.abs(), 0.0, radius);

    let dist_min_z = (z - Complex::new(-radius, -radius)).abs();
    let dist_min_0 = Complex::new(radius, radius).abs();
    let dist_min_max = Complex::new(2.0*radius, 2.0*radius).abs();
    let pos = norm_u8(dist_min_z, dist_min_0, dist_min_max);

    Color::RGB(pos, im, re)
}
