use rayon::prelude::*;

use crate::player::Player;
use crate::board::Board;

const CONNECTED_COMPONENTS_WIN_THRESHOLD: u8 = 4;

#[derive(Debug)]
pub enum WinConditionStrategy {
    HorizontalWinStrategy,
    VerticalWinStrategy,
    DiagonalWinStrategy
}

impl WinConditionStrategy {
    pub fn has_won(&self, player: &Player, board: &Board) -> bool {
        match self {
            WinConditionStrategy::HorizontalWinStrategy => has_won_horizontally(player, board),
            WinConditionStrategy::VerticalWinStrategy => has_won_vertically(player, board),
            WinConditionStrategy::DiagonalWinStrategy => has_won_diagonally(player, board) 
        }
    }    
}


fn has_won_horizontally(player: &Player, board: &Board) -> bool {
    for row in board.data.iter() {
        let mut current_count = 0;
        let mut max_count = 0;
        
        for player_on_board in row.iter() {
            if player_on_board == player {
                current_count += 1;
                max_count = std::cmp::max(max_count, current_count);
                if max_count >= CONNECTED_COMPONENTS_WIN_THRESHOLD {
                    return true;
                }
            } else {
                current_count = 0;
            }
        }
    }
    false
}


fn has_won_vertically(player: &Player, board: &Board) -> bool {
    for col in 0..board.get_cols() {
        let mut current_count = 0;
        let mut max_count = 0;

        for row in 0..board.get_rows() {
            if board.data[row as usize][col as usize] == *player {
                current_count += 1;
                max_count = std::cmp::max(max_count, current_count);
                if max_count >= CONNECTED_COMPONENTS_WIN_THRESHOLD {
                    return true;
                }
            } else {
                current_count = 0;
            }
        }
    }
    false
}


fn has_won_north_east(player: &Player, board: &Board) -> bool {
    let rows = board.get_rows();
    let cols = board.get_cols();
    for row in (CONNECTED_COMPONENTS_WIN_THRESHOLD-1)..rows {
        for col in 0..=cols-CONNECTED_COMPONENTS_WIN_THRESHOLD {
            let mut count = 0;
            for offset in 0..CONNECTED_COMPONENTS_WIN_THRESHOLD {
                if board.data[(row - offset) as usize][(col + offset) as usize] == *player {
                    count += 1;
                } else {
                    break;
                }
            }
            if count == CONNECTED_COMPONENTS_WIN_THRESHOLD {
                return true;
            }
        }
    }
    false
}



fn has_won_south_east(player: &Player, board: &Board) -> bool {
    let rows = board.get_rows();
    let cols = board.get_cols();
    for row in 0..=rows-CONNECTED_COMPONENTS_WIN_THRESHOLD {
        for col in 0..=cols-CONNECTED_COMPONENTS_WIN_THRESHOLD {
            let mut count = 0;
            for offset in 0..CONNECTED_COMPONENTS_WIN_THRESHOLD { // check 1, 2, 3, 4 on a row etc
                if board.data[(row + offset) as usize][(col + offset) as usize] == *player {
                    count += 1;
                } else {
                    break;
                }
            }
            if count == CONNECTED_COMPONENTS_WIN_THRESHOLD {
                return true;
            }
        }
    }
    false
}

fn has_won_diagonally(player: &Player, board: &Board) -> bool {
    has_won_north_east(player, board) || has_won_south_east(player, board)
}

#[cfg(test)]
mod win_condition_strategy_tests {
    use super::*;

    #[test]
    fn has_won_horizontally() {
        let mut p = Player::default();
        p.marker = 'x';
        let b = Board::generate_horizontal_win(&p, 4);
        let horizontal_strategy = WinConditionStrategy::HorizontalWinStrategy;
        assert_eq!(horizontal_strategy.has_won(&p, &b), true);
    }

    #[test]
    fn has_won_vertically() {
        let mut p = Player::default();
        p.marker = 'x';
        let b = Board::generate_vertical_win(&p, 4);
        let horizontal_strategy = WinConditionStrategy::VerticalWinStrategy;
        assert_eq!(horizontal_strategy.has_won(&p, &b), true);
    }

    #[test]
    fn has_won_diagonally_south_east() {
        let mut p = Player::default();
        p.marker = 'x';
        let b = Board::generate_diagonal_south_east_win(&p, 4);
        let diagonal_strategy = WinConditionStrategy::DiagonalWinStrategy;
        assert_eq!(diagonal_strategy.has_won(&p, &b), true);
    }

    #[test]
    fn has_won_diagonally_north_east() {
        let mut p = Player::default();
        p.marker = 'x';
        let b = Board::generate_diagonal_north_east_win(&p, 4);
        let diagonal_strategy = WinConditionStrategy::DiagonalWinStrategy;
        assert_eq!(diagonal_strategy.has_won(&p, &b), true);
    }


    #[test]
    fn game_drawn() {
        let b = Board::generate_full_board();
        let all_strategies = vec![
            WinConditionStrategy::HorizontalWinStrategy,
            WinConditionStrategy::VerticalWinStrategy,
            WinConditionStrategy::DiagonalWinStrategy
        ];
        let mut p = Player::default();
        p.marker = 'x';
        assert_eq!(all_strategies.iter().any(|strategy| strategy.has_won(&p, &b)), false);
    }
}

