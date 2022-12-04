use std::cell::RefCell;
use std::rc::Rc;

use glam::Vec2;
use sdl2::rect::Rect;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, EventPump};
use time::Duration;

use crate::asset_store::AssetStore;
use crate::components::{
    AnimationComponent, AnimationComponentBuilder, BoxColliderComponent,
    BoxColliderComponentBuilder, RigidBodyComponent, SpriteComponent, TransformComponent,
    TransformComponentBuilder,
};
use crate::ecs::command_buffer::CommandBuffer;
use crate::ecs::entities::Entity;
use crate::ecs::events::WorldEventSubscriber;
use crate::ecs::query::Query;
use crate::ecs::world::World;
use crate::logger::Logger;
use crate::map::load_map;
use crate::resources::DeltaTime;
use crate::sdl::{Context, MILLIS_PER_FRAME};
use crate::systems::{AnimationSystem, CollisionSystem, DebugSystem, MovementSystem, RenderSystem, Collision};

pub struct Game<'a> {
    is_running: bool,
    context: Context,
    logger: Logger,
    player: Vec2,
    world: World<'a>,
}

impl Game<'static> {
    pub fn new() -> Self {
        let context = Context::new("My game", 800, 600);

        Self {
            context,
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
            self.world.add_resource(DeltaTime(delta_time));
            self.process_input(&mut event_pump);
            self.update(&delta_time);
            self.render(&delta_time);
        }
    }

    fn load_level(&mut self, level: i32) {
        let texture_creator: TextureCreator<WindowContext> =
            self.context.canvas.borrow().texture_creator();
        let mut asset_store = AssetStore::new(texture_creator);
        asset_store.add_texture(
            "tank".to_owned(),
            "./assets/images/tank-tiger-right.png".to_owned(),
        );
        asset_store.add_texture(
            "truck".to_owned(),
            "./assets/images/truck-ford-left.png".to_owned(),
        );
        asset_store.add_texture(
            "chopper".to_owned(),
            "./assets/images/chopper.png".to_owned(),
        );
        asset_store.add_texture("radar".to_owned(), "./assets/images/radar.png".to_owned());

        asset_store.add_texture(
            "jungle".to_owned(),
            "./assets/tilemaps/jungle.png".to_owned(),
        );

        self.world.add_resource(asset_store);
        let map = load_map("./assets/tilemaps/jungle.map");

        map.tiles.iter().enumerate().for_each(|(i, tile)| {
            let tile_column = *tile % map.tiles_per_file_row;
            let tile_row = *tile / map.tiles_per_file_row;
            let map_column = i as u32 % map.tiles_per_row;
            let map_row = i as u32 / map.tiles_per_row;
            let mut sprite = SpriteComponent::tile(map.tile_size, map.tile_size, "jungle");
            sprite.src = Rect::new(32 * tile_column as i32, 32 * tile_row as i32, 32, 32);
            self.world
                .create_entity()
                .with_component(sprite)
                .with_component(TransformComponent {
                    position: Vec2::new(
                        map_column as f32 * (map.tile_size as f32 * map.tile_scale),
                        map_row as f32 * (map.tile_size as f32 * map.tile_scale),
                    ),
                    scale: Vec2::new(map.tile_scale, map.tile_scale),
                    rotation: 0.0,
                })
                .finish_entity();
        });

        self.world
            .create_entity()
            .with_component(
                TransformComponentBuilder::default()
                    .position(Vec2::new(10.0, 30.0))
                    .build()
                    .unwrap(),
            )
            .with_component(RigidBodyComponent {
                velocity: Vec2::new(50.0, 0.0),
            })
            .with_component(SpriteComponent::enemy(32, 32, "tank"))
            .with_component(
                BoxColliderComponentBuilder::default()
                    .width(32)
                    .height(32)
                    .build()
                    .unwrap(),
            )
            .finish_entity();

        self.world
            .create_entity()
            .with_component(
                TransformComponentBuilder::default()
                    .position(Vec2::new(400.0, 30.))
                    .build()
                    .unwrap(),
            )
            .with_component(RigidBodyComponent {
                velocity: Vec2::new(-50.0, 0.0),
            })
            .with_component(SpriteComponent::enemy(32, 32, "truck"))
            .with_component(
                BoxColliderComponentBuilder::default()
                    .width(32)
                    .height(32)
                    .build()
                    .unwrap(),
            )
            .finish_entity();

        self.world
            .create_entity()
            .with_component(
                TransformComponentBuilder::default()
                    .position(Vec2::new(10.0, 30.0))
                    .scale(Vec2::new(3.0, 3.0))
                    .rotation(90.0)
                    .build()
                    .unwrap(),
            )
            .with_component(RigidBodyComponent {
                velocity: Vec2::new(0.0, 50.0),
            })
            .with_component(SpriteComponent::enemy(32, 32, "chopper"))
            .with_component(
                AnimationComponentBuilder::default()
                    .num_of_frames(2)
                    .frame_rate_speed(15)
                    .start_time(self.context.instant.borrow().elapsed())
                    .build()
                    .unwrap(),
            )
            .finish_entity();

        self.world
            .create_entity()
            .with_component(TransformComponent {
                position: Vec2::new(250.0, 250.0),
                scale: Vec2::new(1.0, 1.0),
                rotation: 0.0,
            })
            .with_component(
                TransformComponentBuilder::default()
                    .position(Vec2::new(250.0, 250.0))
                    .build()
                    .unwrap(),
            )
            .with_component(SpriteComponent::ui(64, 64, "radar"))
            .with_component(
                AnimationComponentBuilder::default()
                    .num_of_frames(8)
                    .frame_rate_speed(6)
                    .start_time(self.context.instant.borrow().elapsed())
                    .build()
                    .unwrap(),
            )
            .finish_entity();
        self.world.add_system(MovementSystem::new(), false);
        self.world
            .add_system(RenderSystem::new(self.context.canvas.clone()), false);
        self.world.add_system(AnimationSystem {
            instant: self.context.instant.clone(),
        }, false);
        self.world.add_system(CollisionSystem::new(), false);

        self.world.events().subscribe(even_handler);
    }

    fn setup(&mut self) {
        self.load_level(1);
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
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                        if self.world.has_system::<DebugSystem>() {
                            self.logger.error("Removing debug system");
                            self.world.remove_system::<DebugSystem>();
                        } else {
                            self.logger.error("Adding debug system");
                            self.world.add_system::<DebugSystem>(DebugSystem::new(self.context.canvas.clone()), true);
                        }
                    }
                _ => {}
            }
        }
    }

    pub fn update(&mut self, delta_time: &Duration) {
        let time_to_wait = MILLIS_PER_FRAME - delta_time.whole_milliseconds() as i32;

        if time_to_wait > 0 && time_to_wait <= MILLIS_PER_FRAME {
            ::std::thread::sleep(std::time::Duration::from_millis(time_to_wait as u64));
        }

        self.world.update();
        self.world.update_system::<MovementSystem>();
        self.world.update_system::<CollisionSystem>();
    }

    pub fn render(&mut self, _: &Duration) {
        self.context
            .canvas
            .borrow_mut()
            .set_draw_color(Color::RGB(21, 21, 21));
        self.context.canvas.borrow_mut().clear();

        self.world.update_system::<AnimationSystem>();
        self.world.update_system::<RenderSystem>();
        self.world.update_system::<DebugSystem>();

        self.context.canvas.borrow_mut().present()
    }
}


fn even_handler(event: &Collision, query: &Query, cmd_buffer: &mut CommandBuffer) {
    cmd_buffer.remove_entity(&Entity(event.a));
    cmd_buffer.remove_entity(&Entity(event.b));
}
