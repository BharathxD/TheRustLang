// Example Syntax
// enum Option<T> {
//     Some(T),
//     None
// }

#[derive(Debug)]
enum UsState {
    Alaska,
    Nebraska,
    Ohio,
    Massachussets,
}

enum Coin {
    Penny(UsState),
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let x: i8 = 1;
    let y: Option<i8> = Some(1);

    let sum = x + y.unwrap_or(0);

    println!("{}", &sum);

    value_in_cents(Coin::Penny(UsState::Massachussets));
}

fn _syntax() {
    let _some_number: Option<i32> = Some(1);
    let _some_string: Option<&str> = Some("Hello, world!");

    let _none: Option<i32> = None;
}

fn value_in_cents(coin: Coin) -> u8 {
    return match coin {
        Coin::Penny(state) => {
            println!("{:?}", state);
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    };
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        _ => None,
    }
}

fn if_let() {
    let some_value: Option<i32> = Some(5);

    // Conditional statement using match
    match some_value {
        Some(5) => println!("Five!"),
        _ => (),
    }

    // Simpler Approach using if let
    if let Some(5) = some_value {
        println!("Five!");
    }
}
