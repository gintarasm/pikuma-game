use std::{cell::RefCell, rc::Rc};

use sdl2::keyboard::Keycode;
use sdl2::pixels;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use time::Instant;

use crate::asset_store::AssetStore;
use crate::components::{
    AnimationComponent, BoxColliderComponent, CameraFollowComponent, KeyboardControlledComponent, SpriteLayer,
};
use secs::command_buffer::CommandBuffer;
use secs::events::{EventEmitter, WorldEventEmmiter};
use secs::query::Query;
use crate::game::{Camera, MapDimensions, self};
use crate::resources::DeltaTime;
use crate::{
    components::{RigidBodyComponent, SpriteComponent, TransformComponent},
    logger::Logger,
};
use secs::{entities::Entity, world::World, System, SystemAction, SystemBuilder};
use self::events::{Collision, KeyPressed};

pub mod events;

pub struct MovementSystem {}

impl MovementSystem {
    pub fn new() -> Self {
        Self {}
    }
}

impl SystemAction for MovementSystem {
    fn action(
        &mut self,
        query: Query,
        entities: &Vec<Entity>,
        _: &mut CommandBuffer,
        _: EventEmitter,
    ) {
        let mut transforms = query.components().get_mut::<TransformComponent>();
        let rigid_bodies = query.components().get::<RigidBodyComponent>();
        let delta_time = query.resources.get::<DeltaTime>().borrow().get::<DeltaTime>().0;

        let mut logger_r = query.resources.get::<Logger>().borrow_mut();
        let mut logger = logger_r.get_mut::<Logger>();

        logger.info(&format!(
            "Movement system updating with entities {}",
            entities.len()
        ));

        for ent in entities {
            let transform = transforms.get_mut(ent.0).unwrap();
            let rigid_body = rigid_bodies.get(ent.0).unwrap();

            transform.position += rigid_body.velocity * delta_time.as_seconds_f32();

            logger.info(&format!(
                "Entity {} new position is now ({}, {})",
                ent.0, transform.position.x, transform.position.y
            ));
        }
    }

    fn to_system(self, world: &World) -> System {
        SystemBuilder::new("MovementSystem", self, world.get_component_signatures())
            .with_component::<TransformComponent>()
            .with_component::<RigidBodyComponent>()
            .build()
    }
}

pub struct RenderSystem {
    context: Rc<RefCell<WindowCanvas>>,
}

impl RenderSystem {
    pub fn new(context: Rc<RefCell<WindowCanvas>>) -> Self {
        Self { context }
    }
}

impl SystemAction for RenderSystem {
    fn action(
        &mut self,
        query: Query,
        entities: &Vec<Entity>,
        _: &mut CommandBuffer,
        _: EventEmitter,
    ) {
        let asset_store_r = query.resources.get::<AssetStore>().borrow();
        let asset_store = asset_store_r.get::<AssetStore>();
        
        let camera_r = query.resources.get::<Camera>().borrow();
        let camera = camera_r.get::<Camera>();

        let transforms = query.components().get::<TransformComponent>();
        let sprites = query.components().get::<SpriteComponent>();
        let mut canvas = self.context.borrow_mut();

        let (mut ui, mut other): (Vec<_>, Vec<_>) = entities
            .iter()
            .map(|entity| {
                (
                    transforms.get(entity.0).unwrap(),
                    sprites.get(entity.0).unwrap(),
                )
            })
            .partition(|(_, sprite)| matches!(sprite.layer, SpriteLayer::Ui(_)));

        other.sort_by(|a, b| a.1.layer.cmp(&b.1.layer));
        ui.sort_by(|a, b| a.1.layer.cmp(&b.1.layer));

        for (transform, sprite) in other {
            let texture = asset_store.get_texture(&sprite.asset_id);
            let src_rect = sprite.src;

            let dst = Rect::new(
                transform.position.x as i32 - camera.rect.x,
                transform.position.y as i32 - camera.rect.y,
                sprite.width * transform.scale.x as u32,
                sprite.height * transform.scale.y as u32,
            );

            canvas
                .copy_ex(
                    &texture,
                    Some(src_rect),
                    Some(dst),
                    transform.rotation as f64,
                    None,
                    false,
                    false,
                )
                .unwrap();
        }

        for (transform, sprite) in ui {
            let texture = asset_store.get_texture(&sprite.asset_id);
            let src_rect = sprite.src;

            let dst = Rect::new(
                transform.position.x as i32,
                transform.position.y as i32,
                sprite.width * transform.scale.x as u32,
                sprite.height * transform.scale.y as u32,
            );

            canvas
                .copy_ex(
                    &texture,
                    Some(src_rect),
                    Some(dst),
                    transform.rotation as f64,
                    None,
                    false,
                    false,
                )
                .unwrap();
        }
    }

    fn to_system(self, world: &World) -> System {
        SystemBuilder::new("RenderSystem", self, world.get_component_signatures())
            .with_component::<TransformComponent>()
            .with_component::<SpriteComponent>()
            .build()
    }
}

pub struct AnimationSystem {
    pub instant: Rc<RefCell<Instant>>,
}

impl SystemAction for AnimationSystem {
    fn action(
        &mut self,
        query: Query,
        entities: &Vec<Entity>,
        _: &mut CommandBuffer,
        _: EventEmitter,
    ) {
        let mut sprites = query.components().get_mut::<SpriteComponent>();
        let mut animations = query.components().get_mut::<AnimationComponent>();

        for entity in entities {
            let sprite = sprites.get_mut(entity.0).unwrap();
            let animation = animations.get_mut(entity.0).unwrap();

            let current_frame_ms = ((self.instant.borrow().elapsed().whole_milliseconds()
                - animation.start_time.whole_milliseconds())
                * animation.frame_rate_speed as i128
                / 1000)
                % animation.num_of_frames as i128;

            animation.current_frame = current_frame_ms as u32;
            sprite
                .src
                .set_x((animation.current_frame * sprite.width) as i32);
        }
    }

    fn to_system(self, world: &World) -> System {
        SystemBuilder::new("AnimationSystem", self, world.get_component_signatures())
            .with_component::<SpriteComponent>()
            .with_component::<AnimationComponent>()
            .build()
    }
}

pub struct CollisionSystem {}

impl CollisionSystem {
    pub fn new() -> Self {
        Self {}
    }
}

impl SystemAction for CollisionSystem {
    fn action(
        &mut self,
        query: Query,
        entities: &Vec<Entity>,
        command_buffer: &mut CommandBuffer,
        emitter: EventEmitter,
    ) {
        let transforms = query.components().get::<TransformComponent>();
        let box_colliders = query.components().get::<BoxColliderComponent>();

        let mut logger_r = query.resources.get::<Logger>().borrow_mut();
        let mut logger = logger_r.get_mut::<Logger>();

        for (i, entity_a) in entities.iter().enumerate() {
            let a_transform = transforms.get(entity_a.0).unwrap();
            let a_collider = box_colliders.get(entity_a.0).unwrap();

            for entity_b in entities[i + 1..].iter() {
                let b_transform = transforms.get(entity_b.0).unwrap();
                let b_collider = box_colliders.get(entity_b.0).unwrap();

                let a_offset = a_transform.position + a_collider.offset;
                let b_offset = b_transform.position + b_collider.offset;

                let collided = check_AABB_collision(
                    a_offset.x as u32,
                    a_offset.y as u32,
                    a_collider.width,
                    a_collider.height,
                    b_offset.x as u32,
                    b_offset.y as u32,
                    b_collider.width,
                    b_collider.height,
                );

                if collided {
                    logger.warn(&format!(
                        "Entity {} and {} collided",
                        entity_a.0, entity_b.0
                    ));

                    emitter.emit(
                        Collision {
                            a: entity_a.0,
                            b: entity_b.0,
                        },
                        command_buffer,
                        &query,
                    );
                }
            }
        }
    }

    fn to_system(self, world: &World) -> System {
        SystemBuilder::new("ColissionSystem", self, world.get_component_signatures())
            .with_component::<BoxColliderComponent>()
            .with_component::<TransformComponent>()
            .build()
    }
}

fn check_AABB_collision(
    a_x: u32,
    a_y: u32,
    a_width: u32,
    a_height: u32,
    b_x: u32,
    b_y: u32,
    b_width: u32,
    b_height: u32,
) -> bool {
    a_x < b_x + b_width && a_x + a_width > b_x && a_y < b_y + b_height && a_y + a_height > b_y
}

pub struct DebugSystem {
    context: Rc<RefCell<WindowCanvas>>,
}

impl DebugSystem {
    pub fn new(context: Rc<RefCell<WindowCanvas>>) -> Self {
        Self { context }
    }
}

impl SystemAction for DebugSystem {
    fn action(
        &mut self,
        query: Query,
        entities: &Vec<Entity>,
        _: &mut CommandBuffer,
        _: EventEmitter,
    ) {


        let camera_r = query.resources.get::<Camera>().borrow();
        let camera = camera_r.get::<Camera>();

        let transforms = query.components().get::<TransformComponent>();
        let colliders = query.components().get::<BoxColliderComponent>();
        let mut canvas = self.context.borrow_mut();

        for entity in entities {
            let transform = transforms.get(entity.0).unwrap();
            let collider = colliders.get(entity.0).unwrap();

            let start = transform.position + collider.offset;

            let collider_rect = Rect::new(
                start.x as i32 - camera.rect.x,
                start.y as i32 - camera.rect.y,
                collider.width * transform.scale.x as u32,
                collider.height * transform.scale.y as u32,
            );

            canvas.set_draw_color(pixels::Color::GREEN);
            canvas.draw_rect(collider_rect).unwrap();
        }
    }

    fn to_system(self, world: &World) -> System {
        SystemBuilder::new("DebugSystem", self, world.get_component_signatures())
            .with_component::<TransformComponent>()
            .with_component::<BoxColliderComponent>()
            .build()
    }
}

pub struct CameraMovementSystem;

impl SystemAction for CameraMovementSystem {
    fn action(
        &mut self,
        query: Query,
        entities: &Vec<Entity>,
        _: &mut CommandBuffer,
        _: EventEmitter,
    ) {

        let map_dimensions_r = query.resources.get::<MapDimensions>().borrow();
        let map_dimensions = map_dimensions_r.get::<MapDimensions>();

        let mut camera_r = query.resources.get::<Camera>().borrow_mut();
        let mut camera = camera_r.get_mut::<Camera>();
        
        let transforms = query.components().get::<TransformComponent>();
        for entity in entities {
            let transform = transforms.get(entity.0).unwrap();

            let transform_x = transform.position.x as i32;
            let transform_y = transform.position.y as i32;

            if transform_x + (camera.rect.w / 2) < map_dimensions.width {
                camera.rect.x = transform_x - (game::WINDOW_WIDTH / 2) as i32;
            }

            if transform_y + (camera.rect.h / 2) < map_dimensions.height {
                camera.rect.y = transform_y - (game::WINDOW_HEIGHT / 2) as i32;
            }

            camera.rect.x = if camera.rect.x < 0 { 0 } else { camera.rect.x };
            camera.rect.x = if camera.rect.x > camera.rect.w { camera.rect.w } else { camera.rect.x };
            camera.rect.y = if camera.rect.y < 0 { 0 } else { camera.rect.y };
            camera.rect.y = if camera.rect.y > camera.rect.h { camera.rect.h } else { camera.rect.y };
        }
    }

    fn to_system(self, world: &World) -> System {
        SystemBuilder::new(
            "CameraMovementSystem",
            self,
            world.get_component_signatures(),
        )
        .with_component::<CameraFollowComponent>()
        .with_component::<TransformComponent>()
        .build()
    }
}

pub fn collision_event_handler(event: &Collision, _: &Query, cmd_buffer: &mut CommandBuffer) {
    cmd_buffer.remove_entity(&Entity(event.a));
    cmd_buffer.remove_entity(&Entity(event.b));
}

pub fn key_pressed_hanlder(event: &KeyPressed, query: &Query, cmd_buffer: &mut CommandBuffer) {
    let mut logger_r = query.resources.get::<Logger>().borrow_mut();
    let mut logger = logger_r.get_mut::<Logger>();

    let keyboard_components = query.components().get::<KeyboardControlledComponent>();
    let mut sprites = query.components().get_mut::<SpriteComponent>();
    let mut rigid_bodies = query.components().get_mut::<RigidBodyComponent>();

    for (id, keyboard_comp) in keyboard_components
        .iter()
        .enumerate()
        .filter(|(_, comp)| comp.is_some())
    {
        let mut sprite = sprites.get_mut(id).unwrap();
        let mut rigid_body = rigid_bodies.get_mut(id).unwrap();
        let keyboard_comp = keyboard_comp.as_ref().unwrap();

        match event.key {
            Keycode::Up => {
                rigid_body.velocity = keyboard_comp.up_velocity;
                sprite.src.y = sprite.height as i32 * 3;
            }
            Keycode::Right => {
                rigid_body.velocity = keyboard_comp.right_velocity;
                sprite.src.y = 0;
            }
            Keycode::Down => {
                rigid_body.velocity = keyboard_comp.down_velocity;
                sprite.src.y = sprite.height as i32;
            }
            Keycode::Left => {
                rigid_body.velocity = keyboard_comp.left_velocity;
                sprite.src.y = sprite.height as i32 * 2;
            }
            _ => {}
        }
    }

    logger.error(&format!("Key pressed {}", event.key));
}
