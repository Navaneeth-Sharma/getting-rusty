pub fn fizbuzz(x: i32) -> String {
    match (x % 3, x % 5) {
        (0, 0) => "FizzBuzz".to_string(),
        (0, _) => "Fizz".to_string(),
        (_, 0) => "Buzz".to_string(),
        (_, _) => format!("{x}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fizbuzz() {
        let fizbuzz_case = fizbuzz(15);
        assert_eq!(fizbuzz_case, "FizzBuzz");

        let fiz_case = fizbuzz(3);
        assert_eq!(fiz_case, "Fizz");

        let buzz_case = fizbuzz(5);
        assert_eq!(buzz_case, "Buzz");

        let other_case = fizbuzz(1);
        assert_eq!(other_case, "1");
    }
}
