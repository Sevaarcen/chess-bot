use core::fmt;
use std::error::Error;

pub mod board;
pub mod pieces;

#[derive(Debug)]
pub enum ChessError {
    InvalidArgument(String),
    InvalidMove(String),
}

impl Error for  ChessError {}

impl fmt::Display for ChessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", &self)
    }
}


pub fn name_to_index_pair(square_name: String) -> Result<(usize, usize), ChessError> {
    if square_name.len() != 2 {
        return Err(ChessError::InvalidArgument(format!("Square name expected, e.g. 'b4', was given: '{}'", square_name)))
    }

    let input_clone = square_name.clone();
    let mut input_chars = input_clone.chars();

    let column_letter = input_chars.next().unwrap();
    if column_letter > 'h' || column_letter < 'a' {
        return Err(ChessError::InvalidArgument(format!("Invalid column reference '{}', must be between 'a' and 'h'", column_letter)))
    }
    let column_index = 7 + column_letter as usize - 'h' as usize;

    let row_number = input_chars.next().unwrap();
    if row_number > '8' || row_number < '1' {
        return Err(ChessError::InvalidArgument(format!("Invalid row reference '{}', must be between '1' and '8'", row_number)))
    }
    let row_index = 7 + row_number as usize - '8' as usize;

    Ok((column_index, row_index))
}

pub fn index_pair_to_name(column: usize, row: usize) -> Result<String, ChessError> {
    if column > 7 {
        return Err(ChessError::InvalidArgument(format!("Column invalid, expected to be between 0-7 inclusive: '{}'", column)))
    }
    let col_char = char::from_u32('a' as u32 + column as u32).unwrap();

    if row > 7 {
        return Err(ChessError::InvalidArgument(format!("Row invalid, expected to be between 0-7 inclusive: '{}'", row)))
    }
    let row_char = char::from_u32('1' as u32 + row as u32).unwrap();

    Ok(format!("{}{}", col_char, row_char))
}