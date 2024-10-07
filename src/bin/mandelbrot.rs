extern crate sdl2;

use core::f32;
use std::i32;

use num::complex::{Complex, Complex32, ComplexFloat};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{self, Color};

use sdl2::gfx::primitives::DrawRenderer;
use sdl2::render::WindowCanvas;

fn bound(n: f32, min: f32, max: f32) -> f32 {
    let mut vals = [min, n, max];
    vals.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    vals[1]
}

fn norm(n: f32, min: f32, max: f32) -> f32 {
    let n_bound = bound(n, min, max);
    (n_bound-min) / (max-min)
}

fn norm_u8(n: f32, min: f32, max: f32) -> u8 {
    (255.0 * norm(n, min, max)) as u8
}

const SCREEN_WIDTH: i16 = 800;
const SCREEN_HEIGHT: i16 = 600;
const WIDTH: usize = SCREEN_WIDTH as usize;
const HEIGHT: usize = SCREEN_HEIGHT as usize;

fn xy2complex(x: f32, y: f32) -> Complex32 {
    let x_min = -2.0;
    let x_max = 1.0;
    let y_min = -2.0;
    let y_max = 2.0;

    Complex::new(x_min + (x_max-x_min) * x / WIDTH as f32,
                 -(y_min + (y_max-y_min) * y / HEIGHT as f32))
}

pub struct Mandelbrot {
    pub divergence: Vec<Vec<i32>>,  // iteration when diverged (if not diverged: i32::MAX)
    pub projections: Vec<Vec<Complex<f32>>>,
    pub cs: Vec<Vec<Complex32>>,
    pub iteration: i32,
}

impl Mandelbrot {
    fn new() -> Mandelbrot {
        let divergence = vec![vec![i32::MAX; WIDTH]; HEIGHT];
        let projections = vec![vec![Complex::new(0.0,0.0); WIDTH]; HEIGHT];
        let mut cs = vec![vec![Complex::new(0.0,0.0); WIDTH]; HEIGHT];

        for y in 0..cs.len() {
            for x in 0..cs[y].len() {
                cs[y][x] = xy2complex((x as i16).into(), (y as i16).into());
            }
        }

        let iteration = 0;

        Mandelbrot {
            divergence,
            projections,
            cs,
            iteration,
        }
    }

    fn iter(&mut self) {
        self.iteration += 1;
        println!("iteration={}", self.iteration);

        let projections = &mut self.projections;
        let divergence = &mut self.divergence;
        for y in 0..projections.len() {
            for x in 0..projections[y].len() {
                if divergence[y][x] > self.iteration {
                    projections[y][x] = projections[y][x] * projections[y][x] + self.cs[y][x];

                    if projections[y][x].abs().is_nan() {  // or greater bounds
                        divergence[y][x] = self.iteration;
                    }
                }
            }
        }
    }

    fn show_divergence<'a>(&self, canvas: &'a mut WindowCanvas) {
        let values = &self.divergence;
        for y in 0..values.len() {
            for x in 0..values[y].len() {
                let v = values[y][x];
                let r = norm_u8(0.0 - v.abs_diff(20) as f32, -10.0, 10.0);
                let g = norm_u8(0.0 - v.abs_diff(30) as f32, -10.0, 10.0);
                let b = norm_u8(v as f32, 0.0, 50.0);
                let color = Color::RGB(r, g, b);
                let _ = canvas.box_(x as i16, y as i16, x as i16, y as i16, color);
            }
        }
        canvas.present();
    }

    fn show_projections<'a>(&self, canvas: &'a mut WindowCanvas) {
        let values = &self.projections;
        for y in 0..values.len() {
            for x in 0..values[y].len() {
                let v = values[y][x];
                let im = norm_u8(v.im, -2.0, 2.0);
                let re = norm_u8(v.re, -2.0, 2.0);
                let color = Color::RGB(0, re, im);
                let _ = canvas.box_(x as i16, y as i16, x as i16, y as i16, color);
            }
        }
        canvas.present();
    }

    fn debug(&self, x: usize, y: usize) {
        println!("");
        println!("y={}, x={}, c={}", y, x, self.cs[y][x]);  // xy2complex((x as i16).into(), (y as i16).into())
        println!("projection={}, abs={}", self.projections[y][x], self.projections[y][x].abs());
        println!("divergence={}", self.divergence[y][x]);
    }

}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsys = sdl_context.video()?;
    let window = video_subsys
        .window(
            "Mandelbrot set",
            (SCREEN_WIDTH as u16).into(),
            (SCREEN_HEIGHT as u16).into(),
        )
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut mandelbrot = Mandelbrot::new();

    let mut events = sdl_context.event_pump()?;

    println!("Press [Space] to show next iteration (projections)…");
    println!("Press [Enter] to show next iteration (divergence)…");
    println!("[Klick] any coordinate for debug output…");
    
    mandelbrot.iter();
    mandelbrot.show_projections(&mut canvas);

    'main: loop {

        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,

                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    if keycode == Keycode::Escape {
                        break 'main;
                    } else if keycode == Keycode::SPACE {
                        mandelbrot.iter();
                        mandelbrot.show_projections(&mut canvas);
                    } else if keycode == Keycode::RETURN {
                        mandelbrot.show_divergence(&mut canvas);
                        mandelbrot.iter();
                    } else if keycode == Keycode::BACKSPACE {
                        mandelbrot.show_projections(&mut canvas);
                    }
                }

                Event::MouseButtonDown { x, y, .. } => {
                    mandelbrot.debug(x.try_into().unwrap(), y.try_into().unwrap());
                }
                _ => {}
            }
        }
    }

    Ok(())
}
