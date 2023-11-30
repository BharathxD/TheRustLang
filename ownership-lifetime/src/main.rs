#[derive(Debug)]
struct Counter {
    count: i32,
}

// Function to print the counter using the Debug trait.
fn print_counter(counter: &Counter) {
    println!("{:?}", counter);
}

// Function to find and return the reference to the Counter with the highest count.
fn highest_counter<'a>(x: &'a Counter, y: &'a Counter) -> &'a Counter {
    // Returns the reference to the Counter with the highest count.
    if x.count > y.count {
        x
    } else {
        y
    }
}

fn main() {
    // Creating instances of the Counter struct.
    let a: Counter = Counter { count: 1 };
    let b: Counter = Counter { count: 2 };

    // Finding the reference to the Counter with the highest count.
    let highest: &Counter = highest_counter(&a, &b);

    // Printing the counter with the highest count.
    print_counter(&highest);
}
