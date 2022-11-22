use std::cell::RefCell;
use std::rc::Rc;

use glam::Vec2;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect, EventPump};
use time::Duration;

use crate::components::{RigidBodyComponent, SpriteComponent, TransformComponent};
use crate::ecs::world::World;
use crate::logger::Logger;
use crate::sdl::{Context, MILLIS_PER_FRAME};
use crate::systems::{MovementSystem, RenderSystem};

pub struct Game<'a> {
    is_running: bool,
    context: Context,
    logger: Logger,
    player: Vec2,
    world: World<'a>,
}

impl Game<'static> {
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
            self.render(&delta_time);
        }
    }

    fn setup(&mut self) {
        let tank = self
            .world
            .create_entity()
            .with_component(TransformComponent {
                position: Vec2::new(10.0, 30.0),
                scale: Vec2::new(1.0, 1.0),
                rotation: 0.0,
            })
            .with_component(RigidBodyComponent {
                velocity: Vec2::new(50.0, 0.0),
            })
            .with_component(SpriteComponent {
                texture: "./assets/images/tank-tiger-right.png".to_owned(),
                height: 32,
                width: 32,
            })
            .finish_entity();

        let tank2 = self
            .world
            .create_entity()
            .with_component(TransformComponent {
                position: Vec2::new(10.0, 30.0),
                scale: Vec2::new(1.0, 1.0),
                rotation: 0.0,
            })
            .with_component(RigidBodyComponent {
                velocity: Vec2::new(0.0, 50.0),
            })
            .with_component(SpriteComponent {
                texture: "./assets/images/tank-tiger-right.png".to_owned(),
                height: 32,
                width: 32,
            })
            .finish_entity();

        self.world.add_system(MovementSystem::new());
        self.world
            .add_system(RenderSystem::new(self.context.canvas.clone()));
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

        self.world.update();
        self.world.update_system::<MovementSystem>(delta_time);
    }

    pub fn render(&mut self, delta_time: &Duration) {
        self.context
            .canvas
            .borrow_mut()
            .set_draw_color(Color::RGB(21, 21, 21));
        self.context.canvas.borrow_mut().clear();

        self.world.update_system::<RenderSystem>(delta_time);

        self.context.canvas.borrow_mut().present()
    }
}
