// Example Syntax
// enum Option<T> {
//     Some(T),
//     None
// }

fn main() {
    let some_number: Option<i32> = Some(1);
    let some_string: Option<&str> = Some("Hello, world!");

    let none: Option<i32> = None;
}
