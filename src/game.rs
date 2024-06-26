use glam::Vec2;
use sdl2::rect::Rect;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, EventPump};
use secs::SystemBuilder;
use time::Duration;

use crate::asset_store::AssetStore;
use crate::components::{
    AnimationComponent, AnimationComponentBuilder, BoxColliderComponent,
    BoxColliderComponentBuilder, CameraFollowComponent, KeyboardControlledComponentBuilder, RigidBodyComponent, SpriteComponent, TransformComponent,
    TransformComponentBuilder,
};
use crate::logger::Logger;
use crate::map::load_map;
use crate::resources::DeltaTime;
use crate::sdl::{Context, MILLIS_PER_FRAME};
use crate::systems::events::KeyPressed;
use crate::systems::{
    collision_event_handler, key_pressed_hanlder,
    AnimationSystem, CameraMovementSystem, CollisionSystem, DebugSystem, MovementSystem,
    RenderSystem,
};
use secs::events::WorldEventSubscriber;
use secs::world::World;

pub const WINDOW_WIDTH: u32 = 800;
pub const WINDOW_HEIGHT: u32 = 600;

pub struct Game<'a> {
    is_running: bool,
    context: Context,
    logger: Logger,
    world: World<'a>,
}

impl Game<'static> {
    pub fn new() -> Self {
        let context = Context::new("My game", WINDOW_WIDTH, WINDOW_HEIGHT);

        Self {
            context,
            is_running: true,
            logger: Logger::new(),
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
            "./assets/images/chopper-spritesheet.png".to_owned(),
        );
        asset_store.add_texture("radar".to_owned(), "./assets/images/radar.png".to_owned());

        asset_store.add_texture(
            "jungle".to_owned(),
            "./assets/tilemaps/jungle.png".to_owned(),
        );

        self.world.add_resource(asset_store);
        self.world.add_resource(Logger::new());
        self.world.add_resource(Camera {
            rect: Rect::new(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT),
        });

        let map = load_map("./assets/tilemaps/jungle.map");

        self.world.add_resource(MapDimensions {
            width: 25 * map.tile_size as i32 * map.tile_scale as i32,
            height: 20 * map.tile_size as i32 * map.tile_scale as i32,
        });

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
                    .rotation(90.0)
                    .build()
                    .unwrap(),
            )
            .with_component(RigidBodyComponent {
                velocity: Vec2::new(0.0, 0.0),
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
            .with_component(
                KeyboardControlledComponentBuilder::default()
                    .up_velocity(Vec2::new(0.0, -100.0))
                    .left_velocity(Vec2::new(-100.0, 0.0))
                    .down_velocity(Vec2::new(0.0, 100.0))
                    .right_velocity(Vec2::new(100.0, 0.0))
                    .build()
                    .unwrap(),
            )
            .with_component(CameraFollowComponent)
            .finish_entity();

        self.world
            .create_entity()
            .with_component(
                TransformComponentBuilder::default()
                    .position(Vec2::new(730.0, 20.0))
                    .build()
                    .unwrap(),
            )
            .with_component(SpriteComponent::ui(64, 64, "radar"))
            .with_component(
                AnimationComponentBuilder::default()
                    .num_of_frames(8)
                    .frame_rate_speed(12)
                    .start_time(self.context.instant.borrow().elapsed())
                    .build()
                    .unwrap(),
            )
            .finish_entity();

        self.world.add_system::<MovementSystem>(
            SystemBuilder::<MovementSystem>::new(self.world.get_component_signatures())
                .with_system_data(MovementSystem)
                .with_action(MovementSystem::action)
                .with_component::<TransformComponent>()
                .with_component::<RigidBodyComponent>()
                .build(),
            false,
        );

        self.world.add_system::<RenderSystem>(
            SystemBuilder::<RenderSystem>::new(self.world.get_component_signatures())
                .with_system_data(RenderSystem::new(self.context.canvas.clone()))
                .with_action(RenderSystem::action)
                .with_component::<TransformComponent>()
                .with_component::<SpriteComponent>()
                .build(),
            false,
        );

        self.world.add_system::<AnimationSystem>(
            SystemBuilder::<AnimationSystem>::new(self.world.get_component_signatures())
                .with_system_data(AnimationSystem::new(self.context.instant.clone()))
                .with_action(AnimationSystem::action)
                .with_component::<SpriteComponent>()
                .with_component::<AnimationComponent>()
                .build(),
            false,
        );

        self.world.add_system::<CollisionSystem>(
            SystemBuilder::<CollisionSystem>::new(self.world.get_component_signatures())
                .with_system_data(CollisionSystem)
                .with_action(CollisionSystem::action)
                .with_component::<TransformComponent>()
                .with_component::<BoxColliderComponent>()
                .build(),
            false,
        );

        self.world.add_system::<CameraMovementSystem>(
            SystemBuilder::<CameraMovementSystem>::new(self.world.get_component_signatures())
                .with_system_data(CameraMovementSystem)
                .with_action(CameraMovementSystem::action)
                .with_component::<CameraFollowComponent>()
                .with_component::<TransformComponent>()
                .build(),
            false,
        );

        self.world.events().subscribe(collision_event_handler);
        self.world.events().subscribe(key_pressed_hanlder);
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
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    if self.world.has_system::<DebugSystem>() {
                        self.logger.error("Removing debug system");
                        self.world.remove_system::<DebugSystem>();
                    } else {
                        self.logger.error("Adding debug system");
                        self.world.add_system::<DebugSystem>(
                            SystemBuilder::<DebugSystem>::new(
                                self.world.get_component_signatures(),
                            )
                            .with_system_data(DebugSystem::new(self.context.canvas.clone()))
                            .with_action(DebugSystem::action)
                            .with_component::<TransformComponent>()
                            .with_component::<BoxColliderComponent>()
                            .build(),
                            false,
                        );
                    }
                }
                Event::KeyDown {
                    keycode: Some(code),
                    ..
                } => self.world.emit_event(KeyPressed { key: code }),
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
        self.world.update_system::<CameraMovementSystem>();
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

pub struct Camera {
    pub rect: Rect,
}

#[derive(Clone)]
pub struct MapDimensions {
    pub height: i32,
    pub width: i32,
}
