fn main() {
    let finish = 5;
    let mut count = 1;
    loop {
        println!("Again!");
        if count >= finish {
            break;
        }
        count += 1; // Glad this increment operator exists!
    }

    let mut number = 4;

    while number != 0 {
        println!("Remaining = {}", number);
        number -= 1;
    }

    let a = [10, 20, 30, 40, 50];

    for e in a.iter() {
        println!("Value is: {}", e);
    }

    // Countdown using a range

    for element in (1..11).rev() {
        println!("{:?}", element );
    }
}
