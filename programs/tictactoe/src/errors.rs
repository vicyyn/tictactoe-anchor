use crate::*;

#[error_code]
pub enum ErrorCode {
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
    #[msg("Wrong dashboard")]
    Wrongdashboard,
    #[msg("Wrong expected state")]
    Gamestate,
    #[msg("Dashboard already initialized")]
    Initialized,
    #[msg("Unexpected value")]
    UnexpectedValue,
    #[msg("Illegal move")]
    Illegalmove,
}
