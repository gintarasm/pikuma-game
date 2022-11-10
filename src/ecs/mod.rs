
pub struct Entity(usize);

pub struct World {}

impl World {
    pub fn create_entity() -> Entity {
        todo!()
    }
}

pub trait System {
    fn update() {
        todo!()
    }
}

pub trait Component: Sized {
    fn id() -> String;
}