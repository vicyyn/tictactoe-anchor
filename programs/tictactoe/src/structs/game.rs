use crate::*;

#[derive(Default)]
#[account]
pub struct Game {
    pub board : [BoardItem;9], 
    pub state : GameState,
    pub player_x: Pubkey,
    pub player_o: Pubkey,
}

impl Game {
    pub fn status(&mut self, x_or_o: BoardItem) {
        let winner =
            // Check rows
            Game::same(x_or_o, &self.board[0..3])
            || Game::same(x_or_o, &self.board[3..6])
            || Game::same(x_or_o, &self.board[6..9])
            // Check columns
            || Game::same(x_or_o, &[self.board[0], self.board[3], self.board[6]])
            || Game::same(x_or_o, &[self.board[1], self.board[4], self.board[7]])
            || Game::same(x_or_o, &[self.board[2], self.board[5], self.board[8]])
            // Check both diagonals
            || Game::same(x_or_o, &[self.board[0], self.board[4], self.board[8]])
            || Game::same(x_or_o, &[self.board[2], self.board[4], self.board[6]]);

        if winner {
            self.state = match x_or_o {
                BoardItem::BoardItemX => GameState::PlayerXWon,
                _ => GameState::PlayerOWon,
            }
        } else if self.board.iter().all(|&p| p != BoardItem::BoardItemEmpty) {
            self.state = GameState::Draw;
        } else {
            if x_or_o == BoardItem::BoardItemX {
                self.state = GameState::PlayerO;
            } else {
                self.state = GameState::PlayerX;
            }
        }
    }

    pub fn same(x_or_o: BoardItem, triple: &[BoardItem]) -> bool {
        triple.iter().all(|&i| matches!(i,x_or_o))
    }

}
