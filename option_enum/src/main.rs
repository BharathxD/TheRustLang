// Example Syntax
// enum Option<T> {
//     Some(T),
//     None
// }

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

fn main() {
    let x: i8 = 1;
    let y: Option<i8> = Some(1);

    let sum = x + y.unwrap_or(0);

    println!("{}", &sum);
}

fn _syntax() {
    let _some_number: Option<i32> = Some(1);
    let _some_string: Option<&str> = Some("Hello, world!");

    let _none: Option<i32> = None;
}

fn value_in_cents(coin: Coin) -> u8 {
    return match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    };
}
