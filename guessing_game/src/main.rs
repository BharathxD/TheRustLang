use colored::*;
use rand::Rng;

fn main() {
    println!("The guessing game!");

    // In Rust, variables are immutable by default, meaning once we give the variable a value, the value won’t change
    let secret_number = rand::thread_rng().gen_range(1, 100);

    loop {
        println!("Enter the number: ");

        // To make a variable mutable, we add mut before the variable name
        // The :: syntax in the ::new line indicates that new is an associated function of the String type
        let mut guess = String::new();

        std::io::stdin()
            .read_line(&mut guess)
            .expect("Something went wrong reading the number");

        // Convert the variable for one type to another, in this case it's String -> u32
        // The parse method on strings converts a string to another type. Here, we use it to convert from a string to a number. We need to
        // tell Rust the exact number type we want by using let guess: u32
        let guess: u32 = match guess.trim().parse() {
            // If user enters correct type, return it
            Ok(num) => num,
            // `_` represents catch_all
            Err(_) => continue,
        };

        println!("Your guess was: {}", guess);

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("{}", "Too small".red()),
            std::cmp::Ordering::Equal => {
                println!("{}", "Equal".green());
                break;
            }
            std::cmp::Ordering::Greater => println!("{}", "Too big".red()),
        }
    }
}
