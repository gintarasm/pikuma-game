use std::{cell::RefCell, rc::Rc};

use sdl2::image::LoadTexture;
use sdl2::render::WindowCanvas;
use sdl2::{pixels::Color, rect::Rect};
use time::Duration;

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
}

impl RenderSystem {
    pub fn new(context: Rc<RefCell<WindowCanvas>>) -> Self {
        Self { context }
    }
}

impl SystemAction for RenderSystem {
    fn action(&mut self, world: &World, entities: &Vec<Entity>, delta_time: &Duration) {
        let transforms = world.query().components().get::<TransformComponent>();
        let sprites = world.query().components().get::<SpriteComponent>();
        let mut canvas = self.context.borrow_mut();

        for ent in entities {
            let transform = transforms.get(ent.0).unwrap();
            let sprite = sprites.get(ent.0).unwrap();

            let texture_creator = canvas.texture_creator();
            let texture = texture_creator
                .load_texture(&sprite.texture)
                .unwrap();

            canvas
                .copy(
                    &texture,
                    None,
                    Some(Rect::new(
                        transform.position.x as i32,
                        transform.position.y as i32,
                        sprite.width,
                        sprite.height
                    )),
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
