#![feature(macro_attributes_in_derive_output)]
use anchor_lang::prelude::*;

mod structs;
mod contexts;
mod errors;

pub use structs::*;
pub use contexts::*;
pub use errors::ErrorCode;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod tictactoe {
    use super::*;


    pub fn initialize_game(ctx: Context<InitializeGame>) -> Result<()> {
        ctx.accounts.process() 
    }

    pub fn player_joins(ctx: Context<PlayerJoins>,) -> Result<()> {
        ctx.accounts.process()
    }

    pub fn player_moves(ctx: Context<PlayerMoves>, player_move:u8) -> Result<()> {
        ctx.accounts.process(player_move)
    }
}


