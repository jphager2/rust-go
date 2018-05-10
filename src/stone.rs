#[derive(Debug)]
pub struct Stone {
    pub color: StoneColor,
}

impl Stone {
    pub fn black_stone() -> Stone {
        Stone {
            color: StoneColor::Black,
        }
    }

    pub fn white_stone() -> Stone {
        Stone {
            color: StoneColor::White,
        }
    }

    pub fn liberty() -> Stone {
        Stone {
            color: StoneColor::Liberty,
        }
    }
}

#[derive(Debug)]
pub enum StoneColor {
    Black,
    White,
    Liberty,
}
