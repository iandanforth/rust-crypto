extern crate rand;

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

    println!("Hello, world!");
}
