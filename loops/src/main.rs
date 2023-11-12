fn main() {
    fibonacci();
}

fn fibonacci() {
    let mut n: String = String::new();
    std::io::stdin().read_line(&mut n).expect("Error reading the input");
    let n = n.trim().parse().expect("Invalid type given as the input in cli");
    let mut t1 = 0i32;
    let mut t2 = 1i32;
    let mut next_val = 0;
    let mut i = 1i32;
    while i <= n {
        if i == 1 {
            println!("0");
            i += 1;
            continue;
        }
        if i == 2 {
            println!("1");
            i += 1;
            continue;
        }
        next_val = t1 + t2;
        t1 = t2;
        t2 = next_val;
        println!("{}", next_val);
        i += 1;
    }
}

fn _countdown() {
    let mut countdown_from = String::new();
    std::io::stdin().read_line(&mut countdown_from).expect("Error reading the input");
    let countdown_from: i32 = countdown_from.trim().parse().expect("Invalid type");
    for count in (0..(countdown_from + 1)).rev() {
        println!("Counting: {}", count);
    }
}
