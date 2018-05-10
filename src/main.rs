extern crate igo;

use std::io;

fn main() {
    let mut board = igo::board(9);
    let mut turn = 0;

    loop {
        igo::print_board(&board);

        let color = if turn % 2 == 0 {
                        igo::black()
                    } else {
                        igo::white()
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
