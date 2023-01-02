use std::{thread, time::Duration};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::{mpsc, RwLock};

use crate::{gamelogic::{pieces::Side, board::ChessBoard, index_pair_to_name, GameEnd}, stratagems::Stratagem};

use super::{Runner, RunnerError};

use windows::Win32::{Foundation::{POINT, WPARAM, LPARAM, LRESULT, HINSTANCE}, UI::WindowsAndMessaging::{SetWindowsHookExA, WH_MOUSE, GetMessageA, MSG, WM_MOUSEFIRST, WM_MOUSELAST, CallNextHookEx, WH_MOUSE_LL, WM_LBUTTONDOWN, MSLLHOOKSTRUCT, PeekMessageA, PM_NOREMOVE, TranslateMessage, UnhookWindowsHookEx}, System::Threading::GetCurrentThreadId};
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::UI::WindowsAndMessaging::{GetCursorPos, DispatchMessageA, PostMessageA};
use windows::Win32::System::Threading::{CreateEventW, SetEvent, WaitForSingleObject};
use windows::Win32::Foundation::CloseHandle;
use screenshots::Screen;


lazy_static! {
    static ref WINDOW_POINTS: RwLock<(POINT, POINT)> = RwLock::new(Default::default());
}

pub struct ChessComGame {
    pub board: ChessBoard,
    window_boundary: (POINT, POINT),
    player_side: Side,
    player_bot: Box<dyn Stratagem>,
    current_turn: Side
}

impl Runner for ChessComGame {
    fn initialize<T: Stratagem + 'static>() -> Result<Self, RunnerError>
        where Self: Sized
    {
        let board = ChessBoard::new();

        println!("Click on the top left of the window");
        let window_boundary = unsafe {
            eprintln!("Hooking low-level mouse events");
            let mouse_hook = SetWindowsHookExA(WH_MOUSE_LL, Some(mouse_event_callback), None, 0).unwrap();
            eprintln!("Created hook: {:?}", mouse_hook);

            // Wait for the first thread message posted from the callback and extract the mouse coordinates from the MSLLHOOKSTRUCT in the message.
            let mut msg_a = MSG::default();
            GetMessageA(&mut msg_a, None, 0, 0).as_bool();
            let top_left_point = (*std::mem::transmute::<isize, *const MSLLHOOKSTRUCT>(msg_a.lParam.0)).pt;  // *danger zone soundtrack starts to play*
            eprintln!("Received point A: {:?}", top_left_point);
            
            // wait for the second message for the bottom right coordinate pair
            let mut msg_b = MSG::default();
            GetMessageA(&mut msg_b, None, 0, 0).as_bool();
            let bottom_right_point = (*std::mem::transmute::<isize, *const MSLLHOOKSTRUCT>(msg_b.lParam.0)).pt;
            eprintln!("Received point B: {:?}", bottom_right_point);

            // once we have our two coordinates, we don't need the mouse hook anymore and can release it
            eprintln!("Releasing mouse hook");
            UnhookWindowsHookEx(mouse_hook);

            (top_left_point, bottom_right_point)
        };

        // get all possible displays
        let all_screens = Screen::all().unwrap();

        // figure out which screen the bounds is actually included on
        let active_screen_opt = all_screens
            .iter()
            .find(
                |s|
                    s.display_info.x <= window_boundary.0.x &&
                    s.display_info.y <= window_boundary.0.y && 
                    (s.display_info.x + s.display_info.width as i32) >= window_boundary.1.x &&
                    (s.display_info.y + s.display_info.height as i32) >= window_boundary.1.y
            );
        
        let active_screen = active_screen_opt.expect("Chosen screen area isn't encompassed by a single screen -- make sure the window is on a single display");
        eprintln!("{:?}", active_screen);
            

        let player_side = Side::White;

        let player_bot = Box::new(<T as Stratagem>::initialize(player_side));

        Ok(Self {
            board,
            window_boundary,
            player_side,
            player_bot,
            current_turn: player_side,
        })
    }

    fn refresh_state(self: &mut Self) -> Result<(), RunnerError> {
        todo!()
    }

    fn execute_bot_move(self: &mut Self) -> Result<(), RunnerError> {
        todo!()
    }

    fn check_victory(self: &Self) -> Option<GameEnd> {
        None  // Just always assume the game isn't over. This is to be lazy since chess.com will check if the game is over for us
        // TODO actually implement this
    }
}


/// References: https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms644986(v=vs.85)
///             https://learn.microsoft.com/en-us/windows/win32/learnwin32/mouse-clicks
///             https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowshookexa?redirectedfrom=MSDN
extern "system" fn mouse_event_callback(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    // when the n_code is negative, CallNextHookEx MUST be called w/o the callback handling the event itself, as per MSDN
    if n_code < 0 {
        unsafe {
            return CallNextHookEx(None, n_code, w_param, l_param)
        }
    }

    if w_param.0 as u32 == WM_LBUTTONDOWN {
        // create message for current thread to intercept and then get the point from.
        unsafe {
            PostMessageA(None, 69, w_param, l_param);  // only the l_param matters or something, I didn't the docs fully but it seems to work
        }
    }

    unsafe {
        return CallNextHookEx(None, n_code, w_param, l_param)
    } 
}