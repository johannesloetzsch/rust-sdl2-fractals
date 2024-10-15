extern crate sdl2;

use std::{cell::RefCell, rc::Rc};


#[derive(Clone)]
pub struct FunctionalSDLConfig {
    pub width: u32,
    pub height: u32,
    pub title: String,
}

impl Default for FunctionalSDLConfig {
    fn default() -> Self {
        FunctionalSDLConfig {width: 800, height: 600, title: String::from("")}
    }
}


#[derive(Clone)]
pub struct FunctionalSDL {
    pub events: Rc<RefCell<sdl2::EventPump>>,
    pub context: sdl2::Sdl,
    pub video: sdl2::VideoSubsystem,
    pub window: sdl2::video::Window,
    pub canvas: Rc<RefCell<sdl2::render::Canvas<sdl2::video::Window>>>,
    pub texture_creator: Rc<sdl2::render::TextureCreator<sdl2::video::WindowContext>>,
}

impl FunctionalSDL {
    pub fn default() -> Self {
        Self::new(FunctionalSDLConfig::default()).unwrap()
    }

    pub fn new(config: FunctionalSDLConfig) -> Result<Self, String> {
        let context = sdl2::init()?;

        let events = Rc::new(RefCell::new(context.event_pump()?));

        let video = context.video()?;

        let window = video
            .window(&config.title, config.width, config.width)
            .position_centered()
            //.opengl()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = Rc::new(RefCell::new(
            window.clone()
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())?
        ));

        let texture_creator = Rc::new(
            canvas.borrow().texture_creator()
        );

        Ok(FunctionalSDL {events, context, video, window, canvas, texture_creator})
    }

    pub fn create_texture(&self, width: u32, height: u32) -> Result<sdl2::render::Texture, String> {
        let pixel_format = sdl2::pixels::PixelFormatEnum::RGB24;
        self.texture_creator.create_texture_streaming(pixel_format, width, height)
            .map_err(|e| e.to_string())
    }

    pub fn map_into_texture(&self, width: u32, height: u32, f: impl Fn(usize) -> [u8; 3]) -> Result<sdl2::render::Texture, String> {
        let pixels: usize = (width*height).try_into().unwrap();
        let pitch: usize = (3*height).try_into().unwrap();
        let mut texture = self.create_texture(width, height)?;
        let rect = None;
        let pixel_data = &(0..pixels).map(f).collect::<Vec<[u8; 3]>>().concat();
        let _ = texture.update(rect, pixel_data, pitch);
        Ok(texture)
    }
}
