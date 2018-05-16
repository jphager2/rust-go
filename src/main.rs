extern crate igo;

use std::io;

fn main() {
    let mut board = igo::board(9);
    let mut turn = 0;

    loop {
        igo::print_board(&board);

        let color = if turn % 2 == 0 {
                        igo::black_stone()
                    } else {
                        igo::white_stone()
                    };
        let mut row = String::new();
        let mut column = String::new();

        println!("{:?}'s turn", color);

        println!("Enter a column: ");
        io::stdin().read_line(&mut column)
            .expect("Failed to read line.");

        println!("Enter a row: ");
        io::stdin().read_line(&mut row)
            .expect("Failed to read line.");

        let row: u32 = match row.trim().parse() {
            Ok(n) => n,
            Err(_) => board.size,
        };

        let mut column_char = b'a';
        for c in column.trim().bytes() {
            column_char = c - 96;
            break;
        }
        let column = column_char as u32;

        let mut rollback = false;

        match board.place(&color, (row, column)) {
            Ok(_) => (),
            Err(e) => {
                rollback = true;
                println!("{}", e);
            }
        };

        for rule in igo.rules {
            match rule.apply(&board, &color, (row,column)) {
                Ok(_) => (),
                Err(e) => {
                    rollback = true;
                    println!("{}", e);
                },
            }
        }

        if rollback {
            board.remove((row, column));
        } else {
            turn += 1;
        }
    }
}
