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
    // Calculates the area of the rectangle.
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    // Checks if the current rectangle can hold another rectangle.
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    // Creating an instance of the Rectangle struct.
    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    // Creating instances of the User struct.
    let user1 = User {
        name: String::from("Bharath"),
        email: String::from("Bharath@vertocity.com"),
    };

    // Creating another User instance using struct update syntax.
    let user2: User = User {
        name: String::from("Lakshman"),
        ..user1
    };

    // Printing the area of the rectangle using the debug macro.
    println!(
        "The area of the rectangle is {} square pixels.",
        dbg!(rectangle.area())
    );
}
