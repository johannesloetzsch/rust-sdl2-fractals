extern crate sdl2;

use std::cmp::min;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{self, Color};

use sdl2::gfx::primitives::DrawRenderer;
use sdl2::render::WindowCanvas;

const SCREEN_WIDTH: i16 = 800;  // i15
const SCREEN_HEIGHT: i16 = 600;  // i15

fn atan(a: f32, b: f32) -> f32 {
    if b == 0.0 {
        return 0.0
    } else if b < 0.0 {
        return (a/b).atan() + std::f32::consts::PI;
    } else {
        return (a/b).atan();
    }
}

/** for each zoomlevel, lines are replaced by 4 new lines of length a/3 **/
fn koch_line(canvas: &WindowCanvas, x0: i16, y0: i16, x4: i16, y4: i16, color: Color, zoom: i16) {

  if zoom == 1 {
      let _ = canvas.line(x0, y0, x4, y4, color);
  } else {
      /* the outer two lines */
      let x1 = x0 + (x4-x0) / 3;
      let y1 = y0 + (y4-y0) / 3;
      let x3 = x0 + 2* (x4-x0) / 3;
      let y3 = y0 + 2* (y4-y0) / 3;
      koch_line(canvas, x0, y0, x1, y1, color, zoom-1);
      koch_line(canvas, x3, y3, x4, y4, color, zoom-1);

      /* the inner two lines (arms of an equilateral triangle) */
      let dx = x3 - x1;
      let dy = y3 - y1;
      let a = ((dx*dx + dy*dy) as f32).sqrt();
      let angle = atan((-dy).into(), dx.into());  // angle of the original line 
      const INTERNAL_ANGLE: f32 = std::f32::consts::FRAC_PI_3;  // 60_f32.to_radians();
      let x2 = x1 + ((a * (angle - INTERNAL_ANGLE).cos()) as i16);
      let y2 = y1 - ((a * (angle - INTERNAL_ANGLE).sin()) as i16);
      koch_line(canvas, x1, y1, x2, y2, color, zoom-1);
      koch_line(canvas, x2, y2, x3, y3, color, zoom-1);
  }
}

fn koch_snowflake(canvas: &WindowCanvas, x: i16, y: i16, a: i16, color: Option<Color>, zoom: i16) {
    let max_zoom = (a as f32).log(3.0).ceil() as i16;
    let effective_zoom = min(zoom, max_zoom);
    println!("zoom: {}/{}", effective_zoom, max_zoom);

    let c = (255 * (effective_zoom-1) / (max_zoom-1)) as u8;
    let color = color.unwrap_or(Color::RGB(c, c, 127+c/2));

    let h = (a as f32 * 3.0_f32.sqrt() / 2.0) as i16;  // height of equilateral triangle

    let x1 = x;
    let y1 = y - h/2;
    let x2 = x - a/2;
    let y2 = y + h/2;
    let x3 = x + a/2;
    let y3 = y + h/2;
    koch_line(&canvas, x1, y1, x2, y2, color, effective_zoom);
    koch_line(&canvas, x2, y2, x3, y3, color, effective_zoom);
    koch_line(&canvas, x3, y3, x1, y1, color, effective_zoom);
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsys = sdl_context.video()?;
    let window = video_subsys
        .window(
            "rust-sdl2_gfx: draw line & FPSManager",
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
    println!("Welcome :)");
    println!("[Klick] into canvas to draw a koch snowflake…");
    println!("Press [Space] to clear the canvas…");
    println!("Press [Backspace] to reset the zoom…");

    let mut events = sdl_context.event_pump()?;

    let mut zoom = 1;

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
                    } else if keycode == Keycode::Space {
                        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
                        canvas.clear();
                        canvas.present();
                    } else if keycode == Keycode::BACKSPACE {
                        zoom = 1
                    }
                }

                Event::MouseButtonDown { x, y, .. } => {
                    koch_snowflake(&canvas, x.try_into().unwrap(), y.try_into().unwrap(), SCREEN_HEIGHT/3, None, zoom);
                    canvas.present();
                    zoom += 1;
                }

                _ => {}
            }
        }
    }

    Ok(())
}
