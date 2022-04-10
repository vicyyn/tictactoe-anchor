use crate::*;

#[derive(Accounts)]
pub struct PlayerJoins<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(mut)]
    pub game : Account<'info,Game>,
}

impl<'info> PlayerJoins<'info> {
    #[access_control(Self::constraints(&self))]
    pub fn process(&mut self) -> Result<()> {
        let Self {
            player,
            game,
            ..
        } = self;

        game.player_o = player.key();
        game.state = GameState::PlayerX;

        Ok(())
    }

     pub fn constraints(&self) -> Result<()> {
        if matches!(self.game.state,GameState::Waiting) {
            return Err(error!(ErrorCode::Gamestate));
        }

        Ok(())
    }
}

