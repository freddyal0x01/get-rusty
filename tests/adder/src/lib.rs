pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err("two plus two does not equal four".to_string())
        }
    }

    #[test]
    fn it_works2() {
        assert_eq!(2 + 2, 4);
    }

    // You can ignore a test using the ignore attribute and adding in the
    // --ignored flag when running the tests
    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }
}
