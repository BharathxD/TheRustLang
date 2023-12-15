fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let chat_list = vec!['y', 'm', 'a', 'q'];
    println!("The largest number is {}", find_largest(number_list));
    println!("The largest char is {}", find_largest(chat_list));
}

fn find_largest<T: PartialOrd + Copy>(vec: Vec<T>) -> T {
    let mut largest = vec[0];

    for number in vec {
        if number > largest {
            largest = number;
        }
    }

    largest
}
