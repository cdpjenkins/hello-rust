

fn add(a: i32, b: i32) -> i32 { a + b }
fn subtract(lhs: i32, rhs: i32) -> i32 { lhs - rhs }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_adds_two_numbers() {
        let result = add(2, 3);
        assert_eq!(result, 5);
    }

    #[test]
    fn subtract_subtracts_two_numbers() {
        let result = subtract(10, 5);
        assert_eq!(result, 5);
    }
}
