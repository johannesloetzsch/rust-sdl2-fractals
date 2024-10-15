extern crate sdl2;

use std::thread;
use std::time::{Duration, Instant};

use fractals::sdl::functional::FunctionalSDL;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use rand::prelude::*;
use sdl2::video::FullscreenType;

pub fn main() -> Result<(), String> {
    let sdl = FunctionalSDL::default();
    let canvas = sdl.clone().canvas;

    let mut speed = 8.0;
    'running: loop {
        let size_rand_component: i32 = rand::thread_rng().gen_range(0..7)-3;
        for base_size in 5..12 {
            let size = (2_i32.pow(base_size) + size_rand_component) as usize;
            let (height, width): (usize, usize) = (size, size);
        
            let start = Instant::now();

            let texture = sdl.map_into_texture(width as u32, height as u32,|i: usize| {
                let y = i / height;
                let x = i % height;
                let r: u8 = ((4*(i%64)+2*y+x)%256) as u8;
                let g: u8 = ((4*((y+x/2)%64))%256) as u8;
                let b: u8 = ((3*(256-i%64)+y+(x*5)/2)%256) as u8;
                [r, g, b]
            })?;
            let render_time = start.elapsed();
            
            let start = Instant::now();
            canvas.borrow_mut().clear();
            for i in 0..50 {
                let s = ((i as f32).powf(1.2) - (i as f32).powf(1.1)).max(size as f32 / canvas.borrow().window().size().0 as f32);
                let (canvas_width, canvas_height) = canvas.borrow().window().size();
                canvas.borrow_mut().copy(&texture, None, Some(Rect::new((canvas_width as i32 - (width as f32 / s) as i32)/2, (canvas_height as i32 - (height as f32 / s) as i32)/2, (width as f32 / s) as u32, ((height as f32) / s) as u32)))?;
            }
            canvas.borrow_mut().present();
            if size % 2 == 0 {
                print!("{}x{} texture", width, height); 
                print!(" rendered in {}ms -> {} FPS", render_time.as_millis(), (1.0/render_time.as_secs_f64()).ceil());
                println!(", presented in {}μs", start.elapsed().as_micros());
            }

            let event_option = sdl.events.borrow_mut().wait_event_timeout(1);
            match event_option {
                None => {},
                Some(event) => {
                    match event {
                        Event::Quit { .. } => break 'running,
                        Event::KeyDown { keycode: Some(keycode), .. } => {
                            match keycode {
                                Keycode::Escape => break 'running,

                                Keycode::F11 => {
                                    if canvas.borrow_mut().window().fullscreen_state() == FullscreenType::Off {
                                        let _ = canvas.borrow_mut().window_mut().set_fullscreen(FullscreenType::Desktop);
                                    } else {
                                         let _ = canvas.borrow_mut().window_mut().set_fullscreen(FullscreenType::Off);
                                    }
                                }

                                Keycode::SPACE  => speed = 0.5,
                                Keycode::PageDown => if speed >= 1.0 {speed = speed / 2.0},
                                Keycode::PAGEUP  => speed *= 2.0,
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
            }
            thread::sleep(Duration::from_millis((1000.0/speed) as u64));
        }
    }
    Ok(())
}
