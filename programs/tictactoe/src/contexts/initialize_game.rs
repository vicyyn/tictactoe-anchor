use crate::*;

#[derive(Accounts)]
pub struct InitializeGame<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(init , payer=payer, space= 9 + 1 + 1 + 32 + 32 + 8)]
    pub game : Account<'info,Game>,
    pub system_program: Program<'info,System>
}

impl<'info> InitializeGame<'_> {
    pub fn process(&mut self) -> Result<()> {
        let Self {
            payer,
            game,
            ..
        } = self;

        game.player_x = payer.key();

        Ok(())
    }
}

