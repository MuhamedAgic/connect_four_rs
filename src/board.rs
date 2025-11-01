use rayon::prelude::*;
use std::fmt;
use crate::player::Player;
use crate::player::PlayerType;

const ROWS: u8 = 6;
const COLS: u8 = 7;

#[derive(Debug)]
pub struct Board {
    pub data: [[Player; COLS as usize]; ROWS as usize],
}


impl Board {
    pub fn new() -> Self {
        Self {
            data: [[Player::default(); COLS as usize]; ROWS as usize]
        }
    }

    pub fn get_available_cols(&self) -> Option<Vec<u8>> {
        let mut available_cols = vec![];
        for i in 0..COLS {
            if self.is_valid_move(i) {
                available_cols.push(i);
            }
        }
        if available_cols.is_empty() {
            return None;
        }
        println!("AVAILABLE = {:?}", available_cols);
        Some(available_cols)
    }
    
    pub fn is_full(&self) -> bool {
        // data[0] is top row
        self.data[0]
            .par_iter()
            .all(|player| *player != Player::default())
    }
    
    pub fn is_column_full(&self, col: u8) -> bool {
        self.data
            .par_iter()
            .all(|row| row[col as usize] != Player::default())
    }
    
    pub fn is_valid_move(&self, col: u8) -> bool {
        col < COLS && !self.is_column_full(col) && !self.is_full()
    }
    
    pub fn apply_gravity(&mut self, col: u8) -> Option<u8> {
        if !self.is_valid_move(col) {
            return None;
        }
        for row in (0..ROWS).rev() {
            if self.data[row as usize][col as usize] == Player::default() {
                return Some(row); // we reached an empty spot
            }
        }
        None
    }

    pub fn clear(&mut self) {
        self.data
            .par_iter_mut()
            .for_each(|row| row
                .iter_mut()
                .for_each(|player| {
                    player.id = 0;
                    player.name = "";
                    player.marker = ' ';
                    player.player_type = PlayerType::COMPUTER;
                })
            );
    }

    pub fn get_rows(&self) -> u8 { ROWS }
    pub fn get_cols(&self) -> u8 { COLS }
    
    pub fn generate_full_board() -> Board {
        let mut b = Board::new();
        let mut p1 = Player::default();
        let mut p2 = Player::default();
        let mut p3 = Player::default();
        let mut p4 = Player::default();
        p1.marker = 'x';
        p2.marker = 'o';
        p3.marker = 'v';
        p4.marker = '@';

        for (i, row) in b.data.iter_mut().enumerate() {
            for (j, player) in row.iter_mut().enumerate() {
                if i % 2 == 0 && j % 2 == 0 {
                    *player = p1;
                } else if i % 2 != 0 && j % 2 == 0 {
                    *player = p2;
                } else if i % 2 == 0 && j % 2 != 0 {
                    *player = p3;
                } else if i % 2 != 0 && j % 2 != 0 {
                    *player = p4;
                }
            }
        }
        println!("Generated full drawn board");
        println!("{}", b);
        b
    }

    
    pub fn generate_horizontal_win(player: &Player, nr_connected_components: u8) -> Board {
        let mut b = Board::new();
        for i in 0..nr_connected_components {
            b.data[ROWS as usize - 1][i as usize] = *player;
        }
        println!("Generated horizontally won board");
        println!("{}", b);
        b
    }

    
    pub fn generate_vertical_win(player: &Player, nr_connected_components: u8) -> Board {
        let mut b = Board::new();
        for i in 0..nr_connected_components {
            b.data[i as usize][COLS as usize - 1] = *player;
        }
        println!("Generated vertically won board");
        println!("{}", b);
        b
    }

    pub fn generate_diagonal_south_east_win(player: &Player, nr_connected_components: u8) -> Board {
        let mut b = Board::new();
        for i in 0..nr_connected_components {
            b.data[i as usize][i as usize] = *player;
        }
        println!("Generated vertically won board");
        println!("{}", b);
        b
    }

    pub fn generate_diagonal_north_east_win(player: &Player, nr_connected_components: u8) -> Board {
        let mut b = Board::new();
        for i in 0..nr_connected_components {
            b.data[ROWS as usize - i as usize - 1][i as usize] = *player;
        }
        println!("Generated vertically won board");
        println!("{}", b);
        b
    }
}


impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " ")?;
        for i in 0..COLS {
            write!(f, " {} ", i)?;
        }
        writeln!(f)?;
        write!(f, " ")?;
        for i in 0..COLS {
            write!(f, " - ")?;
        }
        writeln!(f)?;

        for row in self.data.iter() {
            write!(f, "|")?;
            for player in row.iter() {
                write!(f, " {} ", player.marker)?;
            }
            write!(f, "|")?;
            writeln!(f)?;
        }
        write!(f, " ")?;
        for i in 0..COLS {
            write!(f, " - ")?;
        }
        writeln!(f)?;
        Ok(())
    }
}


#[cfg(test)]
mod board_tests {
    use super::*;

    #[test]
    fn gravity_works() {
        let mut b = Board::new();
        println!("{}", b);
        assert_eq!(b.apply_gravity(0), Some(b.get_rows() - 1)); // on empty board, expect to fall all the way down
        
        let mut p = Player::default();
        p.marker = 'x';
        b.data[ROWS as usize - 1][0] = p; // hardcode player p in first column
        println!("{}", b);
        
        assert_eq!(b.apply_gravity(0), Some(b.get_rows() - 2)); // now expect it to be one higher
    }

    #[test]
    fn board_full_works() {
        let b = Board::generate_full_board();
        assert_eq!(b.is_full(), true);
    }

    #[test]
    fn inserting_full_column_impossible() {
        let mut b = Board::new();
        let mut p1 = Player::default();
        p1.marker = 'x';

        // fill column 0 with players
        for i in 0..b.get_rows() {
            b.data[i as usize][0] = p1;
        }
        
        assert_eq!(b.is_column_full(0), true);
    }


}