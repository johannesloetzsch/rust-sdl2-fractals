use num::complex::{Complex32, ComplexFloat};
use rayon::prelude::*;
use std::time::Instant;
use super::dynamic::HolomorphicDynamic;


pub const D_MAX: i32 = i32::MAX;

/// BOUND defines the threshold, when the simulation should be stopped. For each point outside of the radius of a Fatou
/// set, we know that repeated iteration under f(z) will diverge to infinity. Therefore if BOUND is set >= a critical
/// value depending on f(z), any point with |z| > BOUND will be in the Julia set of f(z). BOUND=2 is sufficient for
/// calculating the classical Mandelbrot set.
///
/// Each iteration will potentiate the number of „external rays“ running from the Julia set or Mandelbrot set towards
/// infinity. When visualizing the state of Julia sets from different iterations, the image will show discontinuities.
/// For rendering smoother images, it is helpful to keep calculating the external rays, even if it is already known that
/// the point is belonging to the Julia set. To prevent overflows, we should set BOUND < D_MAX.nth_root(n), when f(z)
/// is a polynomial of degree n.

const BOUND: f32 = 1.0e10;


pub trait Simulation {
    fn step(&mut self);
    fn benchmark(&mut self, iterations: i32);
}

impl Simulation for HolomorphicDynamic {
    fn step(&mut self) {
        self.i += 1;
        (self.z, self.d) = (0..self.plane.height).into_par_iter().map(|y| {
            (0..self.plane.width).map(|x| {
                if self.d[y][x] != D_MAX {
                    /* once diverged, we don't further mutate */
                    (self.z[y][x], self.d[y][x])
                } else {
                    let c = self.plane.xy_to_c(x, y);  // TODO: memoize
                    let z = (self.f)(self.z[y][x], c);
                    if z.abs() > BOUND {
                        /* this is the iteration of divergence */
                        (z, self.i)
                    } else {
                        /* not yet diverged */
                        (z, D_MAX)
                    }
                }
            }).collect::<(Vec<Complex32>, Vec<i32>)>()
        }).collect::<(Vec<Vec<Complex32>>, Vec<Vec<i32>>)>();
    }

   fn benchmark(&mut self, iterations: i32) {
       println!("Benchmarking {} iterations…", iterations);
       let start = Instant::now();
       for _ in 0..iterations {
           self.step()
       }
       println!("Simulating {} iterations took {}s", iterations, start.elapsed().as_secs());
    }
}

