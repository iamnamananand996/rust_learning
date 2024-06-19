#[derive(Debug)]
enum IPAdress {
    V4(String),
    V6(u8, u8, u8, u8),
}

fn route(ip: IPAdress) -> IPAdress {
    ip
}

enum Coin {
    One,
    Ten,
    Hundred,
    Thousand,
}

fn main() {
    // let four = IPAdress::V4;
    // let six = IPAdress::V6;
    // println!("{:?}", route(IPAdress::V4(String::from("127:0:0:1"))));
    // println!("{:?}", route(IPAdress::V6(127, 0, 0, 1)));

    // let a = 5;
    // let b: Option<i32> = Some(6);

    // println!("{}", a + b.unwrap_or(0))
    println!("{}", value_of_coin(Coin::Ten));
    println!("{:?}", plus_one(None));
}

fn value_of_coin(coin: Coin) -> u32 {
    match coin {
        Coin::One => return 1,
        Coin::Ten => return 10,
        Coin::Hundred => return 100,
        Coin::Thousand => return 1000,
    };
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(x) => Some(x + 1),
        _ => None,
    }
}
