use secs::ecs_macro::GameEvent;
use sdl2::keyboard::Keycode;


#[derive(GameEvent)]
pub struct Collision {
    pub a: usize,
    pub b: usize
}

#[derive(GameEvent)]
pub struct KeyPressed {
    pub key: Keycode
}
