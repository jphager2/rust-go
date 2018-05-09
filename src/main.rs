use std::io;

#[derive(Debug)]
struct Board {
    size: u32,
    description: String,
    board: Vec<Stone>,
}

impl Board {
    fn new(size: u32) -> Board {
        let mut board = Vec::with_capacity((size ^ 2) as usize);
        for _ in 0..size {
            for _ in 0..size {
                board.push(liberty());
            }
        }

        Board {
            size,
            description: String::from(""),
            board,
        }
    }

    fn place(&mut self, color: &StoneColor, coord: (u32, u32)) {
        let i = self.coord_index(coord);
        self.board[i] = if let &StoneColor::Black = color {
                            black_stone()
                        } else {
                            white_stone()
                        }
    }

    fn remove(&mut self, coord: (u32, u32)) {
        let i = self.coord_index(coord);
        self.board[i] = liberty();
    }

    fn coord_index(&self, coord: (u32, u32)) -> usize {
        let row = coord.0;
        let column = coord.1;

        (row * self.size + column) as usize
    }

    fn print(&self) {
        print!("- ");
        for col in 0..self.size {
            print!("{} ", col);
        }
        println!("");
        print!("  ");
        for _ in 0..self.size {
            print!("__");
        }
        println!("");
        for row in 0..self.size {
            print!("{}|", row);
            for col in 0..self.size {
                let i: usize = (row * self.size + col) as usize;
                match self.board[i].color {
                    StoneColor::Black => print!("◯ "),
                    StoneColor::White => print!("⬤ "),
                    StoneColor::Liberty => print!("+ "),
                }
            }
            println!("");
        }
    }
}

#[derive(Debug)]
struct Stone {
    color: StoneColor,
}

#[derive(Debug)]
enum StoneColor {
    Black,
    White,
    Liberty,
}

fn black_stone() -> Stone {
    Stone {
        color: StoneColor::Black,
    }
}

fn white_stone() -> Stone {
    Stone {
        color: StoneColor::White,
    }
}

fn liberty() -> Stone {
    Stone {
        color: StoneColor::Liberty,
    }
}

fn main() {
    let mut board = Board::new(9);
    let mut turn = 0;

    // println!("{:#?}", board);
    loop {
        board.print();

        let color = if turn % 2 == 0 {
                        StoneColor::Black
                    } else {
                        StoneColor::White
                    };
        let mut row = String::new();
        let mut column = String::new();

        println!("{:?}'s turn", color);

        println!("Enter a row: ");
        io::stdin().read_line(&mut row)
            .expect("Failed to read line.");

        println!("Enter a column: ");
        io::stdin().read_line(&mut column)
            .expect("Failed to read line.");

        let row: u32 = match row.trim().parse() {
            Ok(n) => n,
            Err(_) => board.size,
        };
        let column: u32 = match column.trim().parse() {
            Ok(n) => n,
            Err(_) => board.size,
        };

        if row >= board.size {
            println!("Row is off the board");
        }

        if column >= board.size {
            println!("Column is off the board");
        }

        board.place(&color, (row, column));

        turn = turn + 1;

        board.remove((8, 0));
    }
}
