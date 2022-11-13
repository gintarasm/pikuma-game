use glam::Vec2;
use sdl2::image::LoadTexture;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect, EventPump};
use time::Duration;

use crate::components::{RigidBodyComponent, TransformComponent};
use crate::ecs::world::World;
use crate::logger::Logger;
use crate::sdl::{Context, MILLIS_PER_FRAME};

pub struct Game {
    is_running: bool,
    context: Context,
    logger: Logger,
    player: Vec2,
    world: World,
}

impl Game {
    pub fn new() -> Self {
        Self {
            context: Context::new("My game", 800, 600),
            is_running: true,
            logger: Logger::new(),
            player: Vec2::new(10.0, 20.0),
            world: World::new(),
        }
    }

    pub fn run(&mut self) {
        self.logger.info("Starting the game");
        let mut event_pump = self.context.sdl.event_pump().unwrap();
        self.setup();
        while self.is_running {
            let delta_time = self.context.get_delta_time();
            self.process_input(&mut event_pump);
            self.update(&delta_time);
            self.render();
        }
    }

    fn setup(&mut self) {
        let tank = self.world.create_entity();

        self.world.add_component(
            &tank,
            TransformComponent {
                position: Vec2::new(10.0, 30.0),
                scale: Vec2::new(1.0, 1.0),
                rotation: 0.0,
            },
        );

        self.world.add_component(
            &tank,
            RigidBodyComponent {
                velocity: Vec2::ZERO,
            },
        )
    }

    fn process_input(&mut self, event_pump: &mut EventPump) {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.is_running = false,
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {}
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {}
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {}
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {}

                Event::KeyUp {
                    keycode: Some(Keycode::Up),
                    ..
                } => {}
                Event::KeyUp {
                    keycode: Some(Keycode::Down),
                    ..
                } => {}
                Event::KeyUp {
                    keycode: Some(Keycode::Left),
                    ..
                } => {}
                Event::KeyUp {
                    keycode: Some(Keycode::Right),
                    ..
                } => {}
                _ => {}
            }
        }
    }

    pub fn update(&mut self, delta_time: &Duration) {
        let time_to_wait = MILLIS_PER_FRAME - delta_time.whole_milliseconds() as i32;

        if time_to_wait > 0 && time_to_wait <= MILLIS_PER_FRAME {
            ::std::thread::sleep(std::time::Duration::from_millis(time_to_wait as u64));
        }

        let speed = Vec2::new(30.0, 0.0) * delta_time.as_seconds_f32();
        self.player += speed;
    }
    pub fn render(&mut self) {
        self.context.canvas.set_draw_color(Color::RGB(21, 21, 21));
        self.context.canvas.clear();

        let texture_creator = self.context.canvas.texture_creator();
        let texture = texture_creator
            .load_texture("./assets/images/tank-tiger-right.png")
            .unwrap();

        self.context
            .canvas
            .copy(
                &texture,
                None,
                Some(Rect::new(
                    self.player.x as i32,
                    self.player.y as i32,
                    32,
                    32,
                )),
            )
            .unwrap();

        self.context.canvas.present();
    }
}
