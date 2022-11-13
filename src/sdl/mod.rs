use sdl2::{
    render::WindowCanvas,
    Sdl, image::InitFlag,
};
use time::{Duration, Instant};

pub mod buffer;

pub const FPS: i32 = 60;
pub const MILLIS_PER_FRAME: i32 = 1000 / FPS;

pub struct Context {
    pub sdl: Sdl,
    pub canvas: WindowCanvas,
    pub instant: Instant,
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
            // .fullscreen()
            // .borderless()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        Self {
            sdl: sdl,
            canvas: canvas,
            instant: Instant::now(),
            ticks_last_frame: Duration::milliseconds(0),
        }
    }

    pub fn get_delta_time(&mut self) -> Duration {
        let elapsed = self.instant.elapsed();
        let delta_time = elapsed - self.ticks_last_frame;
        self.ticks_last_frame = elapsed;
        delta_time
    }

}
