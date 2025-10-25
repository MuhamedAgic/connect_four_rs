use std::fmt;
use std::io::Write;
use rand::*;
use rand::seq::SliceRandom;

use crate::board::Board;
use crate::utils::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum PlayerType {
    HUMAN,
    COMPUTER
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Player {
    pub id: u8,
    pub player_type: PlayerType,
    pub name: &'static str,
    pub marker: char
}


impl Player {
    
    pub fn new(id: u8, name: &'static str, player_type: PlayerType, marker: char) -> Self {
        Self {id, name, player_type, marker}
    }
    
    pub fn default() -> Self {
        Self { 
            id: 0, 
            name: "", 
            player_type: PlayerType::COMPUTER, 
            marker: ' ' 
        }
    }

    pub fn cli_ask_desired_move(&self, board: &Board) -> Result<u8, String> {
        let input = get_cli_input()
            .trim()
            .parse::<u8>()
            .map_err(|e| e.to_string());
        
        match input {
            Ok(value) => {
                if board.is_valid_move(value) {
                    Ok(value)
                } else {
                    println!("Invalid column {}. Please choose another column", value);
                    self.cli_ask_desired_move(board)
                }
            },
            Err(e) => {
                println!("Error while receiving input: {}", e);
                self.cli_ask_desired_move(board)
            }
        }
    }

    pub fn generate_move(&self, board: &Board) -> Result<u8, String> {
        if let Some(available_cols) = board.get_available_cols() {
            let mut rng = thread_rng();
            if let Some(chosen_col) = available_cols.choose(&mut rng) {
                Ok(*chosen_col)
            } else {
                println!("Could not generate move");
                Err(String::from("Could not generate move"))
            }
        } else {
            println!("No available columns");
            Err(String::from("No available columns"))
        }
    }

    pub fn get_move(&self, board: &Board) -> Result<u8, String> {
        println!("What move would you like to play?");
        match self.player_type {
            PlayerType::HUMAN => self.cli_ask_desired_move(board),
            PlayerType::COMPUTER => self.generate_move(board),
            _ => Err(format!("Unknown player type: {:?}", self.player_type))
        }
    }
    
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.marker)
    }
}


#[cfg(test)]
mod player_tests {
    use super::*;
    
    #[test]
    fn out_of_bound_move_rejected() {
        let mut b = Board::generate_full_board();
        let mut p = Player::default();
        p.marker = 'x';
        assert_eq!(b.apply_gravity(b.get_cols()), None);
    }
    
}