use crate::IpAddr::V4;
use crate::UsState::Alaska;

fn main() {
    let v4 = V4(String::from("127.0.0.1"));
    println!("{:?}", v4);
    Message::Quit.print();
    let some_number = Some(5);

    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    Coin::Quarter(Alaska).value_in_cents();
}

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    //匿名结构体
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn print(&self) {
        print!("{:?}", self);
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

impl Coin {
    fn value_in_cents(&self) -> u32 {
        match self {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("state quarter from {:?}", state);
                25
            }
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(t) => Some(t + 1)
    }
}

fn some_u8_value(x: u8) {
    match x {
        1 => println!("one"),
        2 => println!("two"),
        _ => (),
    }
}

fn match_one(x: Option<u8>) {
    match x {
        Some(1) => println!("one"),
        _ => ()
    }
    //another convenient way
    if let Some(1) = x {
        println!("one");
    } else {
        println!("others")
    }
    let i = 2;
    if let 1 = i {
        println!("1")
    }

    if let Coin::Quarter(state) = Coin::Penny {
        println!("{:?}", state);
    }
}