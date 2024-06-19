fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

mod tests {
    use super::*;

    #[test]
    fn larger_can_holder_smaller() {
        let larger = Rectangle {
            width: 30,
            height: 40,
        };
        let smaller = Rectangle {
            width: 3,
            height: 4,
        };

        assert!(!smaller.can_hold(&larger))
    }
}
