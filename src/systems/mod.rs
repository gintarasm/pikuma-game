use crate::{
    components::{RigidBodyComponent, TransformComponent},
    ecs::{entities::Entity, world::World, System, SystemAction, SystemBuilder},
};

pub struct MovementSystem {}

impl SystemAction for MovementSystem {
    fn action(&self, world: &World, entities: &Vec<Entity>) {
        let mut transforms= world.query().get_mut::<TransformComponent>();
        let rigid_bodies = world.query().get::<RigidBodyComponent>();

        for ent in entities {
            let transform = transforms.get_mut(ent.0).unwrap();
            let rigid_body = rigid_bodies.get(ent.0).unwrap();

            transform.position += rigid_body.velocity;
        }
    }

    fn to_system(self, world: &World) -> System {
        SystemBuilder::new("MovementSystem", self, world.get_component_signatures())
            .with_component::<TransformComponent>()
            .with_component::<RigidBodyComponent>()
            .build()
    }
}
