pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn divide_zero() {
        let a = 1;
        let b = 0;
        divide(a, b);
    }

    #[test]
    fn divide_zero_with_result() -> Result<(), String> {
        let a = 1;
        let b = 1;
        if b == 0 {
            Err(String::from("divide 0"))
        } else {
            divide(a, b);
            Ok(())
        }
    }

    fn divide(a: i32, b: i32) -> i32 {
        a / b
    }
}