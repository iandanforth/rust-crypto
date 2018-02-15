extern crate rand;

use std::io;
use rand::Rng;

fn clear(board: &mut Vec<Vec<i32>>) {
    for i in 0..board.len() {
        let row_len = board[i].len();
        for j in 0..row_len {
            board[i][j] = 0;
        }
    }
}

fn show(board: &Vec<Vec<i32>>) {
    for i in 0..board.len() {
        println!("{:?}", board[i]);
    }
}

fn set_ships(board: &mut Vec<Vec<i32>>, num_ships: i32) {
    let mut s = 0;
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
    if x >= board.len() || y >= board[0].len() {
        println!("Your shot fell outside the board!");
        return false;
    }

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
    let mut num_ships = 5;

    // Populate the board with zeros
    for _i in 0..rows {
        let mut row: Vec<i32> = Vec::new();
        for _j in 0..elements {
            row.push(0);
        }
        board.push(row);
    }

    clear(&mut board);
    set_ships(&mut board, num_ships);

    println!("You are facing a {} by {} board.", rows, elements);
    println!("Somewhere on this board there are {} ships.", num_ships);
    println!("You job is to sink them all with your cannons.");

    loop {
        if num_ships <= 0 {
            println!("All ships have been sunk! You win!");
            show(&board);
            break;
        }
        let mut input = String::new();
        println!("Input your attack coordinates: ");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        let coords: Vec<&str> = input.trim().split(' ').collect();
        if coords.len() != 2 {
            println!("Please enter two coordinates separated by a space. e.g. '2 0'");
            continue;
        }
        let x = coords[0].parse::<usize>().expect("This didn't work!");
        let y = coords[1]
            .parse::<usize>()
            .expect("That didn't work either!");

        if attack(&mut board, x, y) {
            println!("Hit!");
            num_ships -= 1;
            if num_ships > 1 {
                println!("There are {} ships remaining ...", num_ships);
            } else {
                println!("There is 1 ship remaining. Sink that bastard!");
            }
        } else {
            println!("Miss!");
        }
    }
}
