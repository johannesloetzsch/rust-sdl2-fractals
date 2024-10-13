use super::{dynamic::{HolomorphicDynamic, Mandelbrot}, plane::Plane};

pub fn mandelbrot(width: usize, height: usize) -> HolomorphicDynamic {
    let plane = Plane {re_min: -2.0, re_max: 0.55, im_min: -1.2, im_max: 1.2, width, height};
    let f = |z, c| { z*z+c };
    Mandelbrot::new(plane, f)
}
