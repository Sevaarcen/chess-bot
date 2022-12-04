use core::fmt;
use std::error::Error;

use crate::{gamelogic::GameEnd, stratagems::Stratagem};

pub mod local_game;
pub mod chess_com;


/// Different types of Errors related to chess logic specifically. All types wrap String containing a more detailed error message.
#[derive(Debug)]
pub enum RunnerError {
    InitializationFaliure(String),
    ConnectionLost(String),
    UnreadableStateError(String),
    InvalidStateError(String),
    InputLocked(String)
}


impl Error for  RunnerError {}


impl fmt::Display for RunnerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", &self)
    }
}


pub trait Runner {
    fn initialize<T: Stratagem + 'static>() -> Result<Self, RunnerError>
        where Self: Sized;
    fn refresh_state(self: &mut Self) -> Result<(), RunnerError>;
    fn execute_bot_move(self: &mut Self) -> Result<(), RunnerError>;
    fn check_victory(self: &Self) -> Option<GameEnd>;
}

