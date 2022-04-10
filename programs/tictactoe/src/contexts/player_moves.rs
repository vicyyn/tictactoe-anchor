use crate::*;

#[derive(Accounts)]
pub struct PlayerMoves<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(mut)]
    pub game : Account<'info,Game>,
}

impl<'info> PlayerMoves<'info> {
    #[access_control(Self::constraints(&self,player_move))]
    pub fn process(&mut self,player_move:u8) -> Result<()> {
        let Self {
            player,
            game,
            ..
        } = self;

        if player.key() == game.player_x {
            game.board[player_move as usize] = BoardItem::BoardItemX;
            game.status(BoardItem::BoardItemX)
        } else {
            game.board[player_move as usize] = BoardItem::BoardItemO;
            game.status(BoardItem::BoardItemO)
        }
        
        Ok(())
    }

     pub fn constraints(&self,player_move:u8) -> Result<()> {
        if self.game.board[player_move as usize] != BoardItem::BoardItemEmpty {
            return Err(error!(ErrorCode::Illegalmove));
        }

        if self.player.key() != self.game.player_x && self.player.key() != self.game.player_o {
            return Err(error!(ErrorCode::Unauthorized));
        }

        if self.player.key() == self.game.player_x {
           if self.game.state != GameState::PlayerX  {
            return Err(error!(ErrorCode::Unauthorized));
           }
        } else {
           if self.game.state != GameState::PlayerO  {
            return Err(error!(ErrorCode::Unauthorized));
           }
        }

        Ok(())
    }

}

