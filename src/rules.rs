use Board;
use StoneColor;

pub fn apply(mut board: &mut Board, color: &StoneColor, coord: (u32, u32)) -> Result<bool, &'static str> {
    place_stone(&mut board, &color, coord)
}

fn place_stone(board: &mut Board, color: &StoneColor, coord: (u32, u32)) -> Result<bool, &'static str> {
    match board.place(&color, coord) {
        Ok(_) => (),
        Err(_) => return Result::Err("Failed to place stone"),
    };

    Result::Ok(true)
}
