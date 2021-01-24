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

pub fn add_two(a: i32) -> i32 {
    a + 2
}

// custom messages for failures
pub fn greeting(name: &str) -> String {
    // format!("Hello, {}!", name)
    String::from("Hello")
}

pub struct Guess {
    value: i32,
}

pub fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value: {}", a);
    10
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            )
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() -> Result<(), String> {
        // writing tests that return Result<T, E> is convenient because it enables the use of the ? operator, letting tests fail if any operation within them returns an Err variant
        // cant use #[should_panic] with tests that use Result<T, E>. instead, return Err value
        if 2 + 2 == 5 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal five 🤡"))
        }
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 10,
            height: 10,
        };

        let smaller = Rectangle {
            width: 5,
            height: 5,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let smaller = Rectangle {
            width: 1,
            height: 1,
        };

        let larger = Rectangle {
            width: 5,
            height: 5,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
        assert_ne!(5, add_two(1));
    }

    #[test]
    fn adds_two_to_100() {
        assert_eq!(102, add_two(100));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Jesse");
        assert!(
            !result.contains("Jesse"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")] // make sure the panic message matches what is expected, rather than just accepting a panic for any reason. this will pass if the expected message is a substring of the actual message
    fn greater_than_100() {
        Guess::new(420);
    }

    #[test]
    #[should_panic(expected = "Guess value must be greater than or equal to 1")]
    fn less_than_one() {
        Guess::new(0);
    }
    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }

    // run with cargo test -- --show-output so messages sent to stdout are not captured
    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(1);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(11);
        assert_eq!(value, 9);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // long running code
    }
}
