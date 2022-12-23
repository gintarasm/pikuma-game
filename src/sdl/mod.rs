use std::{cell::RefCell, rc::Rc};

use sdl2::{
    render::{WindowCanvas, TextureCreator},
    Sdl, image::InitFlag, video::WindowContext,
};
use time::{Duration, Instant};

pub mod buffer;

pub const FPS: i32 = 60;
pub const MILLIS_PER_FRAME: i32 = 1000 / FPS;

pub struct Context {
    pub sdl: Sdl,
    pub canvas: Rc<RefCell<WindowCanvas>>,
    pub instant: Rc<RefCell<Instant>>,
    pub ticks_last_frame: Duration,
}

impl Context {
    pub fn new(name: &str, height: u32, width: u32) -> Self {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();
        let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();
        let window = video_subsystem
            .window(name, height, width)
            .position_centered()
            //.fullscreen()
            // .borderless()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_blend_mode(sdl2::render::BlendMode::Blend);

        Self {
            sdl: sdl,
            canvas: Rc::new(RefCell::new(canvas)),
            instant: Rc::new(RefCell::new(Instant::now())),
            ticks_last_frame: Duration::milliseconds(0),
        }
    }

    pub fn get_delta_time(&mut self) -> Duration {
        let elapsed = self.instant.borrow().elapsed();
        let delta_time = elapsed - self.ticks_last_frame;
        self.ticks_last_frame = elapsed;
        delta_time
    }

}
