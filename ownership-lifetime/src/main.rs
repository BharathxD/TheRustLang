#[derive(Debug)]
struct Counter {
    count: i32
}

fn print_counter(counter: &Counter) {
    println!("{:?}", counter);
}

fn highest_counter<'a>(x: &'a Counter, y: &'a Counter) -> &'a Counter {
    if x.count > y.count { x } else { y }
}

fn main() {
    let a: Counter = Counter { count: 1 };
    let b: Counter = Counter { count: 2 };
    let highest: &Counter = highest_counter(&a, &b);
    print_counter(&highest);
}
