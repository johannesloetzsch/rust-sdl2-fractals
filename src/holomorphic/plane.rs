use num::{complex::Complex32, Complex, ToPrimitive};
use std::f32::NAN;
use rayon::prelude::*;


/** Rasterized clipping of complex plane **/
#[derive(Clone, Copy)]
pub struct Plane {
    /* viewport */
    pub re_min: f32,
    pub re_max: f32,
    pub im_min: f32,
    pub im_max: f32,
    /* resolution in pixels */
    pub width: usize,
    pub height: usize,
}

impl Plane {
    /** the complex number at a coordinate **/
    pub fn xy_to_c(&self, x: usize, y: usize) -> Complex32 {
        let re = self.re_min + (self.re_max - self.re_min) * x.to_f32().unwrap_or(NAN) / (self.width as f32);
        let im = self.im_min + (self.im_max - self.im_min) * y.to_f32().unwrap_or(NAN) / (self.height as f32);
        Complex::new(re, im)
    }

    /** the complex number for each coordinate **/
    pub fn c(&self) -> Vec<Vec<Complex32>> {
        (0..self.height).into_par_iter().map(|y| {
            (0..self.width).map(|x| {
                self.xy_to_c(x, y)
            }).collect::<Vec<_>>()
        }).collect::<Vec<_>>()
    }
}
