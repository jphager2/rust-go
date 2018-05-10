use stone::{Stone, StoneColor};

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
                board.push(Stone::liberty());
            }
        }

        Board {
            size,
            board,
        }
    }

    pub fn at(&self, coord: (u32, u32)) -> &Stone {
        let i = self.coord_index(coord);
        &self.board[i]
    }

    pub fn place(&mut self, color: &StoneColor, coord: (u32, u32)) {
        let i = self.coord_index(coord);
        self.board[i] = if let &StoneColor::Black = color {
                            Stone::black_stone()
                        } else {
                            Stone::white_stone()
                        }
    }

    pub fn remove(&mut self, coord: (u32, u32)) {
        let i = self.coord_index(coord);
        self.board[i] = Stone::liberty();
    }

    fn coord_index(&self, coord: (u32, u32)) -> usize {
        let row = coord.0;
        let column = coord.1;

        (row * self.size + column) as usize
    }
}
