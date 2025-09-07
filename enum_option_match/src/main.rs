#[derive(Debug)]
enum IpAddr {
 v4s(String),
 v4i(u8, u8, u8, u8),
 v6s(String),
 v6i(u8, u8, u8, u8, u8, u8),
}

fn demo_enum() {
    let home = IpAddr::v4s(String::from("127.0.1.1"));
    let loopBack = IpAddr::v6s(String::from("::01"));
    dbg!(&home);
    dbg!(&loopBack);
}

//variant enum
#[derive(Debug)]
enum Message {
 Quit,
 Move {x:u32, y: i32},
 Write(String),
 ChangeColor{r:u32, g:u32, b:u32},
}

impl Message{
    fn print(&self) -> &Message {
        dbg!(self)
    }
}
fn demo_variant_enum(){
    let msg = Message::Write(String::from("a dummy string"));
    msg.print();
}

#[derive(Debug)]
enum Option1<T>{
    None,
    Some(T),
}


// fn demo_option_enum() {
//     let some_number = Option1::Some(5);
//     let some_char = Option1::Some('e');
//     let absent_number = Option1::<i32>::None;
//     let known_number = Option1::Some(12);

//     // Example: manually unpack and add
//     let res = |v1: Option1<T>, v2: Option1<T>| -> i32 {
//         match (v1, v2) {
//             (Option1::Some(a), Option1::Some(b)) => a + b,
//             (Option1::Some(a), Option1::Some(c)) => a + c as i32,
//             (Option1::Some(c), Option1::Some(b)) => c as i32 + b,
//             (Option1::Some(c1), Option1::Some(c2)) => c1 as i32 + c2 as i32,
//         };
//     }

//     println!("res = {}", res(some_char, some_number));
//     dbg!(some_char);
//     dbg!(absent_number);
// }
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Quarter(state) =>  {
            println!("State quarter from {state:?}!");
            32
        },
        Coin::Dime => todo!() // If called will lead to exception
    }
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            // -- snip --
        }
    }
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        None
    }
}

fn main() {
    println!("Hello, world!");
    demo_enum();
    demo_variant_enum();
    // demo_option_enum();
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alabama)));
    println!("{:#?}", describe_state_quarter(Coin::Quarter(UsState::Alaska)));

}
