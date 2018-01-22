#[derive(Debug)]
enum IPAdressKind {
    V4,
    V6,
}

enum EnumWithTypes {
    V4(u32, u32, u32, u32),
    V6(String),
}

#[derive(Debug)]
struct IP {
    v4: (u8, u8, u8, u8),
    v6: String
}

fn main() {
    
    let four = IPAdressKind::V4;
    let six = IPAdressKind::V6;

    println!("Hello, world!");

    route(four);
    route(six);

    let m = Message::Write(String::from("Well howdy"));
    m.call();
}


fn route(ip_type: IPAdressKind) {
    // unimplemented!();
}

#[derive(Debug)]
enum Message {
    Quit, // No data associated
    Move {x: i32, y: i32 }, // Anonymous struct
    Write(String), // String data
    ChangeColor(i32, i32, i32), // 3 i32 values (Can this be an array?)
}

// Holy fuck you can have methods attached to enums as well and you have to use impl
impl Message {
    fn call(&self) {
        println!("I got called");
    }
}

// Options - These capture when a value either exists or is absent
// You can wrap things in Some() or use None to use the Option<T> type wrapper


// Match

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

fn value_in_cents(coin: Coin) -> i32 {
    match coin {
        Coin::Penny => {
            1
        },
        Coin::Nickel => {
            5
        },
        Coin::Dime => 10, // Anoyingly you can also forgoe the braces and just use this naked fat arrow syntax
        Coin::Quarter => 20,
    }
}

