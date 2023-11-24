#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

struct User {
    name: String,
    email: String,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


fn main() {
    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    let user1 = User {
        name: String::from("Bharath"),
        email: String::from("Bharath@vertocity.com"),
    };

    let user2: User = User {
        name: String::from("Lakshman"),
        ..user1
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        dbg!(rectangle.area())
    );
}
