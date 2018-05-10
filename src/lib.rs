// Lib for the binary

#[derive(Debug)]
pub struct Board {
    pub size: u32,
    board: Vec<Stone>,
}

impl Board {
    pub fn new(size: u32) -> Board {
        let mut board = Vec::with_capacity((size ^ 2) as usize);
        for _ in 0..size {
            for _ in 0..size {
                board.push(liberty());
            }
        }

        Board {
            size,
            board,
        }
    }

    pub fn place(&mut self, color: &StoneColor, coord: (u32, u32)) {
        let i = self.coord_index(coord);
        self.board[i] = if let &StoneColor::Black = color {
                            black_stone()
                        } else {
                            white_stone()
                        }
    }

    pub fn remove(&mut self, coord: (u32, u32)) {
        let i = self.coord_index(coord);
        self.board[i] = liberty();
    }

    fn coord_index(&self, coord: (u32, u32)) -> usize {
        let row = coord.0;
        let column = coord.1;

        (row * self.size + column) as usize
    }

    pub fn print(&self) {
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
pub enum StoneColor {
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


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert!(true, "it was not true");
    }
}
