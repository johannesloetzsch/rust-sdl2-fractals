extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

pub fn main() {
    println!("Hello, world!");
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = match video_subsystem.window("fractals", 800, 600)
        .position_centered()
        .build() {
            Ok(w) => { w },
            Err(_)  => { panic!("Can not build window!") }
        };

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    let mut flip = true;

    'running: loop {
        if i % 255 == 0 {
            flip = !flip;
        }
        if flip && i > 0 {
            i = i - 1;
        } else {
            i = i + 1;
        }
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::from_millis(1000 / 60));
    }
}
