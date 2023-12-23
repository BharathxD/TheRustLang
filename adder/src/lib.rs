#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        // self.width < other.width && self.height > other.height
        self.width > other.width && self.height > other.height
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_function() {
        let larger = Rectangle {
            width: 10,
            height: 10,
        };
        let smaller = Rectangle {
            width: 5,
            height: 5,
        };
        assert!(larger.can_hold(&smaller))
    }

    #[test]
    fn smaller_cannot_hold_function() {
        let larger = Rectangle {
            width: 10,
            height: 10,
        };
        let smaller = Rectangle {
            width: 5,
            height: 5,
        };
        assert!(!smaller.can_hold(&larger))
    }

    #[test]
    // #[should_panic(expected = "assertion failed")]
    fn adds_two() {
        // assert_ne!(add(1, 2), 3);
        // assert!(Something, "add(1, 2) should be 3", add(1, 2));
        assert_eq!(add(1, 2), 3);
    }

    #[should_panic(expected = "assertion failed")]
    fn should_panic() {
        assert!(false, "assertion failed");
    }
}
