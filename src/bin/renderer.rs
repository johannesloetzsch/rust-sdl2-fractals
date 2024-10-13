extern crate sdl2;

use std::thread;
use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use rand::prelude::*;
use sdl2::video::FullscreenType;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        //.opengl()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut events = sdl_context.event_pump()?;
    let mut speed = 8.0;
    'running: loop {
        let size_rand_component = rand::thread_rng().gen_range(0..7)-3;
        for base_size in 5..12 {
            let size = 2_usize.pow(base_size) + size_rand_component;

            let (height, width): (usize, usize) = (size, size);
            let texture_creator = canvas.texture_creator();
            let mut texture = texture_creator
                .create_texture_streaming(PixelFormatEnum::RGB24, width as u32, height as u32)
                .map_err(|e| e.to_string())?;
        
            let start = Instant::now();
            let _r = texture.update(None, &(0..(height*width)).map(|i| {
                let y = i / height;
                let x = i % height;
                let r: u8 = ((4*(i%64)+2*y+x)%256) as u8;
                let g: u8 = ((4*((y+x/2)%64))%256) as u8;
                let b: u8 = ((3*(256-i%64)+y+(x*5)/2)%256) as u8;
                [r, g, b]
            }).collect::<Vec<[u8; 3]>>().concat(), 3*width);
            let render_time = start.elapsed();
            
            let start = Instant::now();
            canvas.clear();
            for i in 0..50 {
                let s = ((i as f32).powf(1.2) - (i as f32).powf(1.1)).max(size as f32 / canvas.window().size().0 as f32);
                canvas.copy(&texture, None, Some(Rect::new((canvas.window().size().0 as i32 - (width as f32 / s) as i32)/2, (canvas.window().size().1 as i32 - (height as f32 / s) as i32)/2, (width as f32 / s) as u32, ((height as f32) / s) as u32)))?;
            }
            canvas.present();
            if size % 2 == 0 {
                print!("{}x{} texture", width, height); 
                print!(" rendered in {}ms -> {} FPS", render_time.as_millis(), (1.0/render_time.as_secs_f64()).ceil());
                println!(", presented in {}μs", start.elapsed().as_micros());
            }

            let event_option = events.wait_event_timeout(1);
            match event_option {
                None => {},
                Some(event) => {
                    match event {
                        Event::Quit { .. } => break 'running,
                        Event::KeyDown { keycode: Some(keycode), .. } => {
                            match keycode {
                                Keycode::Escape => break 'running,

                                Keycode::F11 => {
                                    if canvas.window().fullscreen_state() == FullscreenType::Off {
                                        let _ = canvas.window_mut().set_fullscreen(FullscreenType::Desktop);
                                    } else {
                                         let _ = canvas.window_mut().set_fullscreen(FullscreenType::Off);
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
