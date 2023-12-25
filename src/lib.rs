#[macro_use]
pub mod fizbuzz;
pub mod elu;
pub mod leaky_relu;
pub mod learn;
pub mod map_filter_reduce;
pub mod softmax;

use map_filter_reduce::*;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

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
