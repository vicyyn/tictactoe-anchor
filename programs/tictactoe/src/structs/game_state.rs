use crate::*;

#[derive(Copy,Clone,AnchorSerialize,AnchorDeserialize,PartialEq)]
pub enum GameState {
    Waiting,
    PlayerX,
    PlayerO,
    PlayerXWon,
    PlayerOWon,
    Draw,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Waiting
    }
}
