pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
    // format!("Hello")
}

pub struct Guess {
    pub value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal 1, got {}", value);
        }

        if value > 100 {
            panic!(
                "Guess value must be less than or eqaul to 100, got {}",
                value
            );
        }

        Guess { value }
    }
}

// Cargo will only compile this code if we run "cargo test"
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Guess value must be less than or eqaul to 100")]
    fn greater_than_100() {
        Guess::new(200);

        // Would fail becase of a different message
        // Guess::new(-2);
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Sergei");
        assert!(
            result.contains("Sergei"),
            "Greeting did not contain name, value was `{}`",
            result
        )
    }

    #[test]
    fn it_works() {
        let result = 3 + 2;
        assert_ne!(result, 4);

        let result = 3 + 2;
        assert_eq!(result, 5);
    }

    #[test]
    fn it_works_again() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("2 + 2 doesn't equal 4"))
        }
    }

    // #[test]
    // fn failing_test() {
    //     // It will fail if we panic
    //     // panic!("Make this test fail")
    // }

    // Rectangle test
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller))
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger))
    }

    // Only run occasionally
    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }
}
