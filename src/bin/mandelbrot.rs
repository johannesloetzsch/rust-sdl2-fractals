extern crate sdl2;

use fractals::holomorphic::dynamic::{Juliaset, Mandelbrot};
use fractals::holomorphic::plane::Plane;
use fractals::holomorphic::simulation::Simulation;
use fractals::holomorphic::visualize::Visualize;
use num::complex::Complex;
use sdl2::event::{Event, EventType};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::video::FullscreenType;
use std::cell::RefCell;
use std::io::{stdout, Write};
use std::rc::Rc;


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

    let mut events = sdl_context.event_pump()?;


    println!("Press [F1] and [1] to toggle between Mandelbrot and Julia set…");
    println!("Press [Space] ([F1 or [1]]) to show state z of next iteration…");
    println!("Press [Enter] ([F2] or [2]) to show divergence of next iteration…");
    println!("Press [F11] to toggle fullscreen…");
    println!("[Klick] any coordinate for debug output…");
    println!("Press [Esc] to quit…");

    let mut mandelbrot = {
        let plane = Plane {re_min: -2.0, re_max: 0.55, im_min: -1.2, im_max: 1.2, width: 800, height: 600};
        let f = |z, c| { z*z+c };
        Rc::new(RefCell::new(Mandelbrot::new(plane, f)))
    };

    let mut juliaset = {
        let plane = Plane {re_min: -2.0, re_max: 2.0, im_min: -2.0, im_max: 2.0, width: 800, height: 600};
        let f = |z, _c| { z*z+Complex::new(0.0,1.0 )};
        Rc::new(RefCell::new(Juliaset::new(plane, f)))
    };

    let mut active_dynamic = Rc::clone(&mandelbrot);


    active_dynamic.borrow_mut().step();
    active_dynamic.borrow().visualize_z(&mut canvas);

    'main: loop {
        events.enable_event(EventType::KeyDown);
        let event = events.wait_event();
        events.disable_event(EventType::KeyDown);

        match event {
            Event::Quit { .. } => break 'main,

            Event::KeyDown {
                keycode: Some(keycode),
                ..
            } => {
                if keycode == Keycode::Escape {
                    break 'main;

                } else if keycode == Keycode::F11 {
                    if canvas.window().fullscreen_state() == FullscreenType::Off {
                      let _ = canvas.window_mut().set_fullscreen(FullscreenType::Desktop);
                    } else {
                      let _ = canvas.window_mut().set_fullscreen(FullscreenType::Off);
                    }

                } else if keycode == Keycode::F1 {
                    active_dynamic = Rc::clone(&mandelbrot);
                    active_dynamic.borrow_mut().step();
                    active_dynamic.borrow().visualize_z(&mut canvas);
                } else if keycode == Keycode::Num1 {
                    active_dynamic = Rc::clone(&juliaset);
                    active_dynamic.borrow_mut().step();
                    active_dynamic.borrow().visualize_z(&mut canvas);
                } else if keycode == Keycode::F2 {
                    active_dynamic = Rc::clone(&mandelbrot);
                    active_dynamic.borrow_mut().step();
                    active_dynamic.borrow().visualize_d(&mut canvas);
                } else if keycode == Keycode::Num2 {
                    active_dynamic = Rc::clone(&juliaset);
                    active_dynamic.borrow_mut().step();
                    active_dynamic.borrow().visualize_d(&mut canvas);

                } else if keycode == Keycode::SPACE {
                    active_dynamic.borrow_mut().step();
                    active_dynamic.borrow().visualize_z(&mut canvas);
                } else if keycode == Keycode::RETURN {
                    active_dynamic.borrow_mut().step();
                    active_dynamic.borrow().visualize_d(&mut canvas);
                }
            }

            Event::MouseButtonDown { x, y, .. } => {
                active_dynamic.borrow().debug(x.try_into().unwrap(), y.try_into().unwrap());
            }

            Event::Window { timestamp: _, window_id: _, win_event } => {
                match win_event {
                    sdl2::event::WindowEvent::Resized(w, h) => {
                        canvas.set_draw_color(Color::RGB(0, 0, 0));
                        canvas.clear();
                        canvas.present();

                        {
                            let old_iter = juliaset.borrow().i;
                            let plane = Plane {re_min: -2.0, re_max: 0.55, im_min: -1.2, im_max: 1.2, width: w as usize, height: h as usize};
                            let f = juliaset.borrow().f.clone();
                            juliaset = Rc::new(RefCell::new(Juliaset::new(plane, f)));
                            print!("Recalculating {} iterations of Julia set…", old_iter);
                            let _ = stdout().flush();
                            let mut j = juliaset.borrow_mut();
                            for _ in 0..old_iter { j.step(); }
                            println!(" ✓");
                        }
                        {
                            let old_iter = mandelbrot.borrow().i;
                            let plane = Plane {re_min: -2.0, re_max: 0.55, im_min: -1.2, im_max: 1.2, width: w as usize, height: h as usize};
                            let f = mandelbrot.borrow().f.clone();
                            mandelbrot = Rc::new(RefCell::new(Mandelbrot::new(plane, f)));
                            print!("Recalculating {} iterations of Mandelbrot set…", old_iter);
                            let _ = stdout().flush();
                            let mut m = mandelbrot.borrow_mut();
                            for _ in 0..old_iter { m.step(); }
                            println!(" ✓");
                        }

                        active_dynamic = Rc::clone(&mandelbrot);
                        active_dynamic.borrow().visualize_z(&mut canvas);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    Ok(())
}
