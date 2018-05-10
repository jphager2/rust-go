mod board;
mod stone;

use board::Board;
use stone::StoneColor;

pub fn black() -> StoneColor {
    StoneColor::Black
}

pub fn white() -> StoneColor {
    StoneColor::White
}

pub fn board(size: u32) -> Board {
    Board::new(size)
}

pub fn print_board(board: &Board) {
    print!("- ");
    for col in 0..board.size {
        print!("{} ", col);
    }
    println!("");
    print!("  ");
    for _ in 0..board.size {
        print!("__");
    }
    println!("");
    for row in 0..board.size {
        print!("{}|", row);
        for col in 0..board.size {
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
