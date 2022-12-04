use core::fmt;
use std::error::Error;

use crate::{gamelogic::GameEnd, stratagems::Stratagem};

pub mod local_game;
pub mod chess_com;


/// Different types of Errors related to chess logic specifically. All types wrap String containing a more detailed error message.
#[derive(Debug)]
pub enum ConnectorError {
    InitializationFaliure(String),
    ConnectionLost(String),
    UnreadableStateError(String),
    InvalidStateError(String),
    InputLocked(String)
}


impl Error for  ConnectorError {}


impl fmt::Display for ConnectorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", &self)
    }
}


pub trait Connector {
    fn initialize(strat: Box<dyn Stratagem>) -> Result<Self, ConnectorError>
        where Self: Sized;
    fn refresh_state(self: &mut Self) -> Result<(), ConnectorError>;
    fn execute_bot_move(self: &mut Self) -> Result<(), ConnectorError>;
    fn check_victory(self: &Self) -> Option<GameEnd>;
}

