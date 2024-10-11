use num::{complex::Complex32, Complex};
use super::{r#fn::HolomorphicFn, plane::Plane, simulation::D_MAX};


#[derive(Clone)]
pub struct HolomorphicDynamic {
    pub plane: Plane,
    pub f: Box<dyn HolomorphicFn>,
    pub z: Vec<Vec<Complex32>>,  // state
    pub d: Vec<Vec<i32>>,        // diverged
    pub i: i32,                  // iteration
}

impl HolomorphicDynamic {
    fn new(plane: Plane, f: impl HolomorphicFn + 'static, z0: Vec<Vec<Complex32>>) -> Self {
        let d = vec![vec![D_MAX; plane.width]; plane.height];
        Self {plane, f: Box::new(f), z: z0, d, i: 0}
    }
}


pub struct Juliaset;

impl Juliaset {
    pub fn new(plane: Plane, f: impl HolomorphicFn + 'static) -> HolomorphicDynamic {
        let z0 = plane.c();
        HolomorphicDynamic::new(plane, f, z0)
    }
}


pub struct Mandelbrot;

impl Mandelbrot {
    pub fn new(plane: Plane, f: impl HolomorphicFn + 'static) -> HolomorphicDynamic {
        let z0 = vec![vec![Complex::new(0.0,0.0); plane.width]; plane.height];
        HolomorphicDynamic::new(plane, f, z0)
    }
}
