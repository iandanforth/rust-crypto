extern crate rand;

use rand::Rng;

fn clear(board: &mut Vec<Vec<i32>>) {
    for i in 0..board.len() {
        let row_len = board[i].len();
        for j in 0..row_len {
            board[i][j] = 0;
        }
    }
}

fn show(board: & Vec<Vec<i32>>) {
    for i in 0..board.len() {
        println!("{:?}", board[i]);
    }   
}

fn set_ships(board: &mut Vec<Vec<i32>>) {
    let mut s = 0;
    let num_ships = 5;
    while s < num_ships {

        let x = rand::thread_rng().gen_range(0, board.len());
        let y = rand::thread_rng().gen_range(0, board.len());
        if board[x][y] == 0 {
            s += 1;
            board[x][y] = 1;
        }
    }
}

fn attack(board: &mut Vec<Vec<i32>>, x: usize, y: usize) -> bool {
    if board[x][y] == 1 {
        board[x][y] = 2;
        return true;
    } else {
        return false;
    }
}

fn main() {

    let mut board: Vec<Vec<i32>> = Vec::new();
    let rows = 5;
    let elements = 5;

    for _i in 0..rows {
        let mut row: Vec<i32> = Vec::new();
        for _j in 0..elements {
            row.push(0);
        }
        board.push(row);
    }

    clear(&mut board);
    show(& board);
    println!("---------------");
    set_ships(&mut board);
    show(& board);

    println!("Hello, world!");
}
