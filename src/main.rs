mod game;
mod sdl;
mod logger;
mod ecs;
mod components;
mod systems;
mod asset_store;

use game::Game;

fn main() {
    let mut game = Game::new();
    game.run(); 
}
