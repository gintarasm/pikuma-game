mod game;
mod sdl;
mod logger;
mod components;
mod systems;
mod asset_store;
mod map;
mod resources;

use game::Game;

#[macro_use]
extern crate derive_builder;

fn main() {
    let mut game = Game::new();
    game.run(); 
}
