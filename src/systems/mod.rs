use std::{cell::RefCell, rc::Rc};

use sdl2::image::LoadTexture;
use sdl2::render::WindowCanvas;
use sdl2::{pixels::Color, rect::Rect};
use time::{Duration, Instant};

use crate::asset_store::{self, AssetStore};
use crate::components::{AnimationComponent, BoxColliderComponent};
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
    fn action(&mut self, world: &World, entities: &Vec<Entity>, delta_time: &Duration) {
        let mut transforms = world.query().components().get_mut::<TransformComponent>();
        let rigid_bodies = world.query().components().get::<RigidBodyComponent>();
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
    asset_store: Rc<RefCell<AssetStore>>,
}

impl RenderSystem {
    pub fn new(context: Rc<RefCell<WindowCanvas>>, asset_store: Rc<RefCell<AssetStore>>) -> Self {
        Self {
            context,
            asset_store,
        }
    }
}

impl SystemAction for RenderSystem {
    fn action(&mut self, world: &World, entities: &Vec<Entity>, delta_time: &Duration) {
        let transforms = world.query().components().get::<TransformComponent>();
        let sprites = world.query().components().get::<SpriteComponent>();
        let mut canvas = self.context.borrow_mut();

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
            let texture_creator = canvas.texture_creator();
            let texture = self.asset_store.borrow().get_texture(&sprite.asset_id);
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
    fn action(&mut self, world: &World, entities: &Vec<Entity>, _: &Duration) {
        let mut sprites = world.query().components().get_mut::<SpriteComponent>();
        let mut animations = world.query().components().get_mut::<AnimationComponent>();

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

impl SystemAction for CollisionSystem {
    fn action(&mut self, world: &World, entities: &Vec<Entity>, _: &Duration) {
        let transforms = world.query().components().get::<TransformComponent>();
        let box_colliders = world.query().components().get::<BoxColliderComponent>();

        for (i, check_entity) in entities.iter().enumerate() {
            let a_transform = transforms.get(check_entity.0).unwrap();
            let a_collider = box_colliders.get(check_entity.0).unwrap();

            for entity in entities[i + 1..].iter() {
                let b_transform = transforms.get(entity.0).unwrap();
                let b_collider = box_colliders.get(entity.0).unwrap();

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

                if (collided) {
                    self.logger.warn(&format!(
                        "Entity {} and {} collided",
                        check_entity.0, entity.0
                    ));
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
