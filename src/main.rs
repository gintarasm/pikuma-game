mod game;
mod sdl;
mod logger;
mod ecs;
mod components;
mod systems;

use game::Game;

fn main() {
    let mut game = Game::new();
    game.run(); 
}
