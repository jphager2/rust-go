mod board;
mod stone;

use board::Board;
use stone::StoneColor;

pub fn black_stone() -> StoneColor {
    StoneColor::Black
}

pub fn white_stone() -> StoneColor {
    StoneColor::White
}

pub fn board(size: u32) -> Board {
    Board::new(size)
}

pub fn print_board(board: &Board) {
    print!("- ");
    for col in 1..(board.size + 1) {
        print!("{} ", ((col + 96) as u8) as char);
    }
    println!("");
    print!("  ");
    for _ in 1..(board.size + 1) {
        print!("__");
    }
    println!("");
    for row in 1..(board.size + 1) {
        print!("{}|", row);
        for col in 1..(board.size + 1) {
            match board.at((row, col)).color {
                StoneColor::Black => print!("◯ "),
                StoneColor::White => print!("⬤ "),
                StoneColor::Liberty => print!("+ "),
            }
        }
        println!("");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert!(true, "it was not true");
    }
}
