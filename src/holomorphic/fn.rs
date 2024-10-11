use num::complex::Complex32;


pub trait HolomorphicFn: Fn(Complex32, Complex32) -> Complex32 + Sync {
    fn clone_box(&self) -> Box<dyn HolomorphicFn>;
}

impl<F> HolomorphicFn for F
where
    F: Fn(Complex32, Complex32) -> Complex32 + Sync + 'static + Clone,
{
    fn clone_box(&self) -> Box<dyn HolomorphicFn + 'static>
    {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn HolomorphicFn> {
    fn clone(&self) -> Self {
        (**self).clone_box()
    }
}
