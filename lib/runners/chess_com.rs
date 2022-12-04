use crate::{gamelogic::{pieces::Side, board::ChessBoard, index_pair_to_name, GameEnd}, stratagems::Stratagem};

use super::{Runner, RunnerError};

use windows::Win32::Foundation::POINT;
use windows::Win32::UI::WindowsAndMessaging::GetCursorPos;
use windows::Win32::System::Threading::{CreateEventW, SetEvent, WaitForSingleObject};
use windows::Win32::Foundation::CloseHandle;


pub struct ChessComGame {
    pub board: ChessBoard,
    window_boundary: ((usize, usize), (usize, usize)),
    player_side: Side,
    player_bot: Box<dyn Stratagem>,
    current_turn: Side
}

impl Runner for ChessComGame {
    fn initialize<T: Stratagem + 'static>() -> Result<Self, RunnerError>
        where Self: Sized
    {
        
        println!("Click on the top left of the window");
        unsafe {
            let event = CreateEventW(None, true, false, None).unwrap();
            SetEvent(event).ok().unwrap();
            WaitForSingleObject(event, 0);
            CloseHandle(event).ok().unwrap();
            println!("{:#?}", event);
        }
        
        todo!()
    }

    fn refresh_state(self: &mut Self) -> Result<(), RunnerError> {
        todo!()
    }

    fn execute_bot_move(self: &mut Self) -> Result<(), RunnerError> {
        todo!()
    }

    fn check_victory(self: &Self) -> Option<GameEnd> {
        todo!()
    }
}