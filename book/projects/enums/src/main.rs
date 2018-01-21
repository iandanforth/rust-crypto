#[derive(Debug)]
enum IPAdressKind {
    V4,
    V6,
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
}


fn route(ip_type: IPAdressKind) {
    // unimplemented!();
}