use crate::board::Board;

mod win_condition_strategy;
mod player;
mod game;
mod board;
mod utils;

use game::Game;


fn main() -> Result<(), String> {
    let mut game = Game::new(
        Board::new(),
        Game::generate_players(),
        Game::setup_win_condition_strategies()
    );
    
    game.run()?;
    Ok(())
}
