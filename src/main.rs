mod game;
mod sdl;
mod logger;
mod ecs;
mod components;
mod systems;
mod asset_store;
mod map;

use game::Game;

#[macro_use]
extern crate derive_builder;

fn main() {
    let mut game = Game::new();
    game.run(); 
}
