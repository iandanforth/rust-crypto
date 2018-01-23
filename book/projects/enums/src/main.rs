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

    value_in_cents2(Coin2::Quarter(UsState::California));
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
            return 1;
        },
        Coin::Nickel => {
            return 5;
        },
        Coin::Dime => 10, // Anoyingly you can also forgoe the braces and just use this naked fat arrow syntax
        Coin::Quarter => 20,
    }
}

#[derive(Debug)]
enum UsState {
    California,
    Maine,
}

#[derive(Debug)]
enum Coin2 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
    HalfDollar
}

fn value_in_cents2(coin: Coin2) -> i32 {
    match coin {
        Coin2::Penny => 1,
        Coin2::Nickel => 5,
        Coin2::Dime => 10,
        Coin2::Quarter(state) => { // Here the value stored in Quarter will be bound to the variable 'state'
            println!("Quarter from {:?}", state);
            return 25;
        },
        _ => 0 // This is "default" option if nothing else matches
    }
}

