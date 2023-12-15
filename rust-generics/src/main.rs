fn main() {
    struct_generic_example();
    vec_generic_example();
    enum_example();

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<U, K> Point<U, K> {
    fn x(&self) -> &U {
        &self.x
    }
}

impl Point<f64, f64> {
    fn y(&self) -> &f64 {
        &self.y
    }
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn enum_example() {
    enum Options<T> {
        Some(T),
        None,
    }
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
}

fn struct_generic_example() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!("integer.x = {}", integer.x);
    println!("float.x = {}", float.x);
}

fn vec_generic_example() {
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
