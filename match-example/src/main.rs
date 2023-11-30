fn main() {
    // Age variable representing an individual's age
    let age: i32 = 8;

    // Match statement to categorize age ranges
    match age {
        1..=18 => println!("Less than or equal to 18"),
        40 | 50 => println!("The age is 40 (or) 50"),
        65..=i32::MAX => println!("The age is above 65"),
        _ => println!("Not sure"),
    }
}
