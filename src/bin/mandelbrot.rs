extern crate sdl2;

use fractals::coloring::gradient::gradient_rgb;
use rayon::prelude::*;

use core::f32;
use std::i32;

use num::complex::{Complex, Complex32, ComplexFloat};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use sdl2::gfx::primitives::DrawRenderer;
use sdl2::render::WindowCanvas;
use sdl2::video::FullscreenType;

use fractals::coloring::domain_coloring::domain_coloring;

fn xy2complex(x: f32, y: f32, w: usize, h: usize) -> Complex32 {
    let x_min = -2.0;
    let x_max = 0.55;
    let y_min = -1.2;
    let y_max = 1.2;

    Complex::new(x_min + (x_max-x_min) * x / w as f32,
                 -(y_min + (y_max-y_min) * y / h as f32))
}

pub struct Mandelbrot {
    pub divergence: Vec<Vec<i32>>,  // iteration when diverged (if not diverged: i32::MAX)
    pub projections: Vec<Vec<Complex<f32>>>,
    pub cs: Vec<Vec<Complex32>>,
    pub iteration: i32,
    pub width: usize,
    pub height:usize,
}

impl Mandelbrot {
    fn new(width: usize, height: usize) -> Mandelbrot {
        let divergence = vec![vec![i32::MAX; width]; height];
        let projections = vec![vec![Complex::new(0.0,0.0); width]; height];
        let mut cs = vec![vec![Complex::new(0.0,0.0); width]; height];

        for y in 0..cs.len() {
            for x in 0..cs[y].len() {
                cs[y][x] = xy2complex((x as i16).into(), (y as i16).into(), width, height);
            }
        }

        let iteration = 0;

        Mandelbrot {
            divergence,
            projections,
            cs,
            iteration,
            width,
            height
        }
    }

    fn iter(&mut self) {
        self.iteration += 1;
        println!("iteration={}", self.iteration);

        (self.projections, self.divergence) = (0..self.height).into_par_iter().map(|y| {
            (0..self.width).map(|x| {
                if self.divergence[y][x] < self.iteration {
                    (self.projections[y][x], self.divergence[y][x])
                } else {
                    let p = self.projections[y][x]*self.projections[y][x] + self.cs[y][x];
                    let d = if self.projections[y][x].abs().is_nan() {  // or greater bounds
                                self.iteration
                            } else {
                                self.divergence[y][x]
                            };
                    (p, d)
                }
            }).collect::<(Vec<Complex32>, Vec<i32>)>()
        }).collect::<(Vec<Vec<Complex32>>, Vec<Vec<i32>>)>();
    }

    fn show_divergence<'a>(&self, canvas: &'a mut WindowCanvas) {
        for y in 0..self.height {
            for x in 0..self.width {
                let color = gradient_rgb(self.divergence[y][x]);
                let _ = canvas.pixel(x as i16, y as i16, color);
            }
        }
        canvas.present();
    }

    fn show_projections<'a>(&self, canvas: &'a mut WindowCanvas) {
        for y in 0..self.height {
            for x in 0..self.width {
                let color = domain_coloring(self.projections[y][x], 2.0);
                let _ = canvas.pixel(x as i16, y as i16, color);
            }
        }
        canvas.present();
    }

    fn show_combination<'a>(&self, canvas: &'a mut WindowCanvas) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.divergence[y][x] < self.iteration {
                    let color = gradient_rgb(self.divergence[y][x]);
                    let _ = canvas.pixel(x as i16, y as i16, color);
                } else {
                    let color = domain_coloring(self.projections[y][x], 2.0);
                    let _ = canvas.pixel(x as i16, y as i16, color);
                }
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
            800,
            600,
        )
        .resizable()
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut mandelbrot = Mandelbrot::new(800, 600);

    let mut events = sdl_context.event_pump()?;

    println!("Press [Space] to show next iteration…");
    println!("Press [Enter] to show state Z of next iteration…");
    println!("Press [Backspace] to show divergence d of next iteration…");
    println!("Press [F11] to toggle fullscreen…");
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
                        mandelbrot.show_combination(&mut canvas);
                    } else if keycode == Keycode::RETURN {
                        mandelbrot.iter();
                        mandelbrot.show_projections(&mut canvas);
                    } else if keycode == Keycode::BACKSPACE {
                        mandelbrot.iter();
                        mandelbrot.show_divergence(&mut canvas);
                    } else if keycode == Keycode::F11 {
                        if canvas.window().fullscreen_state() == FullscreenType::Off {
                          let _ = canvas.window_mut().set_fullscreen(FullscreenType::Desktop);
                        } else {
                          let _ = canvas.window_mut().set_fullscreen(FullscreenType::Off);
                        }
                    }
                }

                Event::MouseButtonDown { x, y, .. } => {
                    mandelbrot.debug(x.try_into().unwrap(), y.try_into().unwrap());
                }

                Event::Window { timestamp: _, window_id: _, win_event } => {
                    match win_event {
                        sdl2::event::WindowEvent::Resized(w, h) => {
                            canvas.set_draw_color(Color::RGB(0, 0, 0));
                            canvas.clear();
                            canvas.present();
                            let old_iter = mandelbrot.iteration;
                            mandelbrot = Mandelbrot::new(w as usize, h as usize);
                            for _ in 0..old_iter {
                                mandelbrot.iter();
                                mandelbrot.show_combination(&mut canvas);
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    Ok(())
}
