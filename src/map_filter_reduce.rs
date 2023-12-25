// Map Exercises
pub fn square_numbers(numbers: Vec<i32>) -> Vec<i32> {
    // Your code here
    numbers.into_iter().map(|x| x.pow(2)).collect()
}

pub fn double_numbers(numbers: Vec<i32>) -> Vec<i32> {
    // Your code here
    numbers.into_iter().map(|x| x * 2).collect()
}

// Filter Exercises
pub fn even_numbers(numbers: Vec<i32>) -> Vec<i32> {
    // Your code here
    numbers.into_iter().filter(|&x| x % 2 == 0).collect()
}

pub fn positive_numbers(numbers: Vec<i32>) -> Vec<i32> {
    // Your code here
    numbers.into_iter().filter(|&x| x > 0).collect()
}

// Reduce Exercises
pub fn sum_numbers(numbers: Vec<i32>) -> i32 {
    // Your code here
    numbers.into_iter().reduce(|a, b| a + b).unwrap()
}

pub fn product_numbers(numbers: Vec<i32>) -> i32 {
    // Your code here
    numbers.into_iter().reduce(|a, b| a * b).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_filter_reduce() {
        let numbers_map = vec![1, 2, 3, 4, 5];
        assert_eq!(square_numbers(numbers_map.clone()), vec![1, 4, 9, 16, 25]);
        assert_eq!(double_numbers(numbers_map.clone()), vec![2, 4, 6, 8, 10]);

        // Filter Exercises
        let numbers_filter = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(even_numbers(numbers_filter.clone()), vec![2, 4, 6, 8, 10]);
        assert_eq!(
            positive_numbers(numbers_filter.clone()),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        );

        // Reduce Exercises
        let numbers_reduce = vec![1, 2, 3, 4, 5];
        assert_eq!(sum_numbers(numbers_reduce.clone()), 15);
        assert_eq!(product_numbers(numbers_reduce.clone()), 120);
    }
}
