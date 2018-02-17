use std::process::Command;
use std::thread;
use std::time;

fn clear_screen() {
    let output = Command::new("clear").output().unwrap_or_else(|e| {
        panic!("failed to execute process: {}", e)
    });
    print!("{}", String::from_utf8_lossy(&output.stdout));
}

fn sleep() {
    let frame_delay = time::Duration::from_millis(33);
    thread::sleep(frame_delay);    
}

fn tick() {
    sleep();
    clear_screen();
}

fn show(board: &Vec<Vec<i32>>) {
    for i in 0..board.len() {
        println!("{:?}", board[i]);
    }
}

fn main() {
    clear_screen();

    let mut counter = 0;

    let mut board: Vec<Vec<i32>> = Vec::new();

    let board_dims = 10;
    for _i in 0..board_dims {
        let mut row: Vec<i32> = Vec::new();
        for _j in 0..board_dims {
            row.push(0);
        }
        board.push(row);
    }

    loop {
        show(&board);
        println!("{:?}", counter);
        counter += 1;
        tick();
    }
}