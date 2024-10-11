use crate::coloring::{domain_coloring::domain_coloring, gradient::gradient_rgb};
use sdl2::{gfx::primitives::DrawRenderer, render::WindowCanvas};
use super::dynamic::HolomorphicDynamic;


pub trait Visualize {
    fn visualize_z(&self, canvas: &mut WindowCanvas);
    fn visualize_d(&self, canvas: &mut WindowCanvas);
    fn debug(&self, x: usize, y: usize);
}

impl Visualize for HolomorphicDynamic {
    fn visualize_z<'a>(&self, canvas: &'a mut WindowCanvas) {
        /* we assume the first divergence happened in one of the opposing corners or not yet */
        let first_divergence = self.d[0][0].min(self.d[self.plane.height-1][self.plane.width-1]).min(1);
        let s_steps = 20.0;
        for y in 0..self.plane.height {
            for x in 0..self.plane.width {
                let s = (0.9 / s_steps) * ((self.d[y][x]-(first_divergence+1)) as f32).min(s_steps);
                let color = domain_coloring(self.z[y][x], 2.0, s);
                let _ = canvas.pixel(x as i16, y as i16, color);
            }
        }
        canvas.present();
    }

    fn visualize_d<'a>(&self, canvas: &'a mut WindowCanvas) {
        for y in 0..self.plane.height {
            for x in 0..self.plane.width {
                let color = gradient_rgb(self.d[y][x]);
                let _ = canvas.pixel(x as i16, y as i16, color);
            }
        }
        canvas.present();
    }

    fn debug(&self, x: usize, y: usize) {
        println!("y={}, x={}, i={}", x, y, self.i);
        let c = self.plane.xy_to_c(x, y);
        let (c_r, c_theta) = c.to_polar();
        println!("c(x,y) = {} = {}*e^i*{}", c, c_r, c_theta);
        let z = self.z[y][x];
        let (z_r, z_theta) = z.to_polar();
        println!("z(x,y) = {} = {}*e^i*{}, d(x,y) = {}", z, z_r, z_theta, self.d[y][x]); 
    }
}
