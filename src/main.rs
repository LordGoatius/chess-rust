use chess::Game;

use crate::game::MyGame;

pub mod game;

fn main() {
    let game = Game::new();
    let mut mygame = MyGame(game);
    mygame.game_loop();
}
