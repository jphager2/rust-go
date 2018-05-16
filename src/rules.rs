use Board;
use StoneColor;

pub fn apply(board: &Board, color: &StoneColor, coord: (u32, u32)) -> Result<bool, str> {
    match place_stone(&board, &color, coord) {
        Ok(_) => (),
        Err(_) => return Result::Err("Failed to place stone"),
    };

    Result::Ok(true)
}

fn place_stone(board: &Board, color: &StoneColor, coord: (u32, u32)) -> Result<bool, str> {
    match board.place(&color, coord) {
        Ok(_) => (),
        Err(_) => return Result::Err("Failed to place stone"),
    };

    Result::Ok(true)
}
