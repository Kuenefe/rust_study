enum IPAddrKind{
    V4,
    V6,
}

enum IPAddr_ {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum UsSate {
    Alabama,
    Alaska,
    California,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsSate),
}

impl Coin {
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}", state);
                25
            }
        }
    }
}

#[derive(Debug)]
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}


impl Message {
    fn call(&self) {
        println!("{:?}", self)
    }
}

struct IPAdrr {
    kind: IPAddrKind,
    address: String,
}

fn main() {
    let four: IPAddrKind = IPAddrKind::V4;
    let six: IPAddrKind = IPAddrKind::V6;

    let home: IPAdrr = IPAdrr {
        kind: IPAddrKind::V4,
        address: String::from("home"),
    };

    let loopback: IPAdrr = IPAdrr {
        kind: IPAddrKind::V6,
        address: String::from("::1"),
    };

    let home2: IPAddr_ = IPAddr_::V4(127,0,0,1);
    let loopback2: IPAddr_ = IPAddr_::V6(String::from("::1"));

    let msg: Message = Message::Write(String::from("hello"));

    msg.call();

    let some_number: Option<i32> = Some(5);
    let some_char: Option<char> = Some('e');

    let absent_number: Option<i32> = None;

    let coin: Coin = Coin::Quarter(UsSate::California);

    let result: u8 = Coin::value_in_cents(coin);

    println!("{:?}", result);

    let five: Option<i32> = Some(5);
    let six: Option<i32> = plus_one(five);
    let none: Option<i32> = plus_one(None);

    let dice_roll: u8 = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

}

fn route(ip_kind: IPAddrKind) {}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}