use std::io;

/// Calculates and prints the Fibonacci sequence up to the specified term.
///
/// # Examples
///
/// ```
/// fibonacci();
/// ```
fn fibonacci() {
    // Read input from the user
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Error reading the input");

    // Parse input to an integer
    let n: i32 = n.trim().parse().expect("Invalid type given as the input in CLI");

    // Initialize Fibonacci sequence variables
    let mut t1 = 0i32;
    let mut t2 = 1i32;
    let mut next_val = 0;
    let mut i = 1i32;

    // Calculate and print the Fibonacci sequence
    while i <= n {
        match i {
            1 => println!("0"),
            2 => println!("1"),
            _ => {
                next_val = t1 + t2;
                t1 = t2;
                t2 = next_val;
                println!("{}", next_val);
            }
        }
        i += 1;
    }
}

/// Performs a countdown from the user-specified number to zero.
///
/// # Examples
///
/// ```
/// _countdown();
/// ```
fn _countdown() {
    // Read input from the user
    let mut countdown_from = String::new();
    io::stdin().read_line(&mut countdown_from).expect("Error reading the input");

    // Parse input to an integer
    let countdown_from: i32 = countdown_from.trim().parse().expect("Invalid type");

    // Perform countdown and print each count
    for count in (0..=countdown_from).rev() {
        println!("Counting: {}", count);
    }
}

fn main() {
    fibonacci();
}
