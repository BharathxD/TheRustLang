// Example Syntax
// enum Option<T> {
//     Some(T),
//     None
// }

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
