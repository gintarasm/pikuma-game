use std::collections::VecDeque;
use std::{cell::RefCell, rc::Rc};

use ecs_macro::GameEvent;
use sdl2::image::LoadTexture;
use sdl2::pixels;
use sdl2::render::WindowCanvas;
use sdl2::{pixels::Color, rect::Rect};
use time::{Duration, Instant};

use crate::asset_store::{self, AssetStore};
use crate::components::{AnimationComponent, BoxColliderComponent};
use crate::ecs::command_buffer::CommandBuffer;
use crate::ecs::events::{EventEmitter, WorldEventEmmiter};
use crate::ecs::query::Query;
use crate::resources::DeltaTime;
use crate::{
    components::{RigidBodyComponent, SpriteComponent, TransformComponent},
    ecs::{entities::Entity, world::World, System, SystemAction, SystemBuilder},
    logger::Logger,
    sdl::Context,
};

pub struct MovementSystem {
    logger: Logger,
}

impl MovementSystem {
    pub fn new() -> Self {
        Self {
            logger: Logger::new(),
        }
    }
}

impl SystemAction for MovementSystem {
    fn action(&mut self, query: Query, entities: &Vec<Entity>, _: &mut CommandBuffer, emitter: EventEmitter) {
        let mut transforms = query.components().get_mut::<TransformComponent>();
        let rigid_bodies = query.components().get::<RigidBodyComponent>();
        let delta_time = query.resources().get::<DeltaTime>().0;
        self.logger.info(&format!(
            "Movement system updating with entities {}",
            entities.len()
        ));

        for ent in entities {
            let transform = transforms.get_mut(ent.0).unwrap();
            let rigid_body = rigid_bodies.get(ent.0).unwrap();

            transform.position += rigid_body.velocity * delta_time.as_seconds_f32();

            self.logger.info(&format!(
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
        Self {
            context,
        }
    }
}

impl SystemAction for RenderSystem {
    fn action(&mut self, query: Query, entities: &Vec<Entity>, _: &mut CommandBuffer, emitter: EventEmitter) {
        let transforms = query.components().get::<TransformComponent>();
        let sprites = query.components().get::<SpriteComponent>();
        let mut canvas = self.context.borrow_mut();
        let asset_store = query.resources().get::<AssetStore>();

        let mut components = entities
            .iter()
            .map(|entity| {
                (
                    transforms.get(entity.0).unwrap(),
                    sprites.get(entity.0).unwrap(),
                )
            })
            .collect::<Vec<(&TransformComponent, &SpriteComponent)>>();

        components.sort_by(|a, b| a.1.layer.cmp(&b.1.layer));

        for (transform, sprite) in components {
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
    fn action(&mut self, query: Query, entities: &Vec<Entity>, _: &mut CommandBuffer, emitter: EventEmitter) {
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

pub struct CollisionSystem {
    logger: Logger,
}

impl CollisionSystem {
    pub fn new() -> Self {
        Self {
            logger: Logger::new(),
        }
    }
}


#[derive(GameEvent)]
pub struct Collision {
    pub a: usize,
    pub b: usize
}

impl SystemAction for CollisionSystem {
    fn action(&mut self, query: Query, entities: &Vec<Entity>, command_buffer: &mut CommandBuffer, emitter: EventEmitter) {
        let transforms = query.components().get::<TransformComponent>();
        let box_colliders = query.components().get::<BoxColliderComponent>();

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
                    self.logger.warn(&format!(
                        "Entity {} and {} collided",
                        entity_a.0, entity_b.0
                    ));
                    
                    emitter.emit(Collision {a: entity_a.0, b: entity_b.0}, command_buffer, &query);
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
    a_x < b_x + b_width && a_x + b_width > b_x && a_y < b_y + b_height && a_y + a_height > b_y
}


pub struct DebugSystem {
    context: Rc<RefCell<WindowCanvas>>,
}

impl DebugSystem {
    pub fn new(context: Rc<RefCell<WindowCanvas>>) -> Self {
        Self {
            context,
        }
    }
}

impl SystemAction for DebugSystem {
    fn action(&mut self, query: Query, entities: &Vec<Entity>, _: &mut CommandBuffer, emitter: EventEmitter) {
        let transforms = query.components().get::<TransformComponent>();
        let colliders = query.components().get::<BoxColliderComponent>();
        let mut canvas = self.context.borrow_mut();

        for entity in entities {
            let transform = transforms.get(entity.0).unwrap();
            let collider = colliders.get(entity.0).unwrap();

            let start = transform.position + collider.offset;

            let collider_rect = Rect::new(start.x as i32, start.y as i32, collider.width, collider.height);

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
