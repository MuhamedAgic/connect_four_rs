use rayon::prelude::*;
use crate::player::{Player, PlayerType};
use crate::board::Board;
use crate::utils::{cli_confirms, get_cli_input};
use crate::win_condition_strategy::WinConditionStrategy;

#[derive(Debug, Eq, PartialEq)]
enum TurnOutcome {
    ContinueGame,
    ExitGame,
    NewGame,
    InvalidMove
}

#[derive(Debug)]
pub struct Game {
    board: Board,
    players: Vec<Player>,
    win_condition_strategies: Vec<WinConditionStrategy>,
}


impl Game {
    pub fn new(board: Board,
               players: Vec<Player>,
               win_condition_strategies: Vec<WinConditionStrategy>) -> Self {
        Self {
            board,
            players,
            win_condition_strategies
        }
    }
    
    fn create_simulated_game() -> Self {
        Game::new(
            Board::new(),
            Game::generate_simulation_players(),
            Game::setup_win_condition_strategies()
        )
    }

    pub fn generate_players() -> Vec<Player> {
        vec![
            Player::new(1, "henk-one", PlayerType::HUMAN, 'x'),
            Player::new(2, "henk-two", PlayerType::COMPUTER, 'o'),
        ]
    }

    pub fn generate_simulation_players() -> Vec<Player> {
        vec![
            Player::new(1, "henk-one", PlayerType::COMPUTER, 'x'),
            Player::new(2, "henk-two", PlayerType::COMPUTER, 'o'),
        ]
    }
    
    pub fn setup_win_condition_strategies() -> Vec<WinConditionStrategy> {
        vec![
            WinConditionStrategy::HorizontalWinStrategy,
            WinConditionStrategy::VerticalWinStrategy,
            WinConditionStrategy::DiagonalWinStrategy,
        ]
    }
    

    pub fn run(&mut self) ->Result<(), String> {
        println!("Welcome to connect four!");
        println!("{}", &self.board);

        let mut turn_outcome = TurnOutcome::ContinueGame;
        let mut player_index = 0;

        while turn_outcome != TurnOutcome::ExitGame {
            let player = &self.players[player_index].clone();
            loop {
                turn_outcome = self.process_turn(player)?;
                if turn_outcome != TurnOutcome::InvalidMove {
                    break;
                }
            }
            
            match turn_outcome {
                TurnOutcome::ExitGame => break,
                TurnOutcome::ContinueGame => {
                    player_index += 1;
                    player_index = player_index % self.players.len();
                    println!("{}", &self.board);
                    continue;
                },
                TurnOutcome::InvalidMove => {
                    println!("Invalid move by: {}", player);
                    continue;
                },
                TurnOutcome::NewGame => {
                    self.reset_game();
                    player_index = 0; // start with player 1 again
                }
            }
            println!("{}", &self.board);
        }
        println!("Game ended!");
        Ok(())
    }


    fn process_turn(&mut self, player: &Player) -> Result<TurnOutcome, String> {
        let chosen_col_move = player.get_move(&self.board)?;
        if let Some(corresponding_row_move) = self.board.apply_gravity(chosen_col_move) {
            self.board.data[corresponding_row_move as usize][chosen_col_move as usize] = player.clone();
            println!("Player moved (row, column): ({}, {})", corresponding_row_move, chosen_col_move);
            println!("{}", &self.board);

            let mut game_over = false;
            if self.has_won(player) {
                println!("Player {} won!", player.name);
                game_over = true;
            }
            
            if self.board.is_full() && !self.has_won(player) {
                println!("It's a draw!");
                game_over = true;
            }
            
            if game_over {
                print!("\nDo you wish to start a new game? y/n: ");
                if cli_confirms() {
                    return Ok(TurnOutcome::NewGame);
                } else {
                    return Ok(TurnOutcome::ExitGame);
                }
            }
            Ok(TurnOutcome::ContinueGame)
        } else {
            Ok(TurnOutcome::InvalidMove)
        }
    }


    fn reset_game(&mut self) {
        println!("Resetting game...");
        self.board.clear();
    }

    
    fn has_won(&self, player: &Player) -> bool {
        self.win_condition_strategies
            .par_iter()
            .any(|strategy| strategy.has_won(player, &self.board))
    }
    
    
    fn has_winner(&self) -> bool {
        self.players
            .par_iter()
            .any(|player| self.has_won(player))
    }
    
}


#[cfg(test)]
mod game_tests {
    use super::*;

    #[test]
    fn full_board_has_no_winner() {
        let mut game = Game::create_simulated_game();
        game.board = Board::generate_full_board();
        assert_eq!(game.has_winner(), false);
    }
    
    #[test]
    fn game_has_winner() {
        let mut game = Game::create_simulated_game();
        
        let boards_with_winner = vec![
            Board::generate_horizontal_win(&game.players[0], 4),
            Board::generate_vertical_win(&game.players[0], 4),
            Board::generate_diagonal_north_east_win(&game.players[0], 4),
            Board::generate_diagonal_south_east_win(&game.players[0], 4)
        ];
    
        for board in boards_with_winner {
            game.board = board;
            assert_eq!(game.has_winner(), true);
        }
    }

    #[test]
    fn legal_move_continues_game() {
        let mut game = Game::create_simulated_game();
        let player = &game.players[0].clone();
        let turn_outcome = &game.process_turn(player);
        assert_eq!(*turn_outcome, Ok(TurnOutcome::ContinueGame));
    }
    
}

