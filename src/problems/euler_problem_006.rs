// The sum of the squares of the first ten natural numbers is,
// 1^2 + 2^2 + ... + 10^2 = 385
//
// The square of the sum of the first ten natural numbers is,
// (1+2+...+10)^2 = 55^2 = 3025
//
// Hence the difference between the sum of the squares of the first ten
// natural numbers and the square of the sum is
// 3025 - 385 = 2640
//
// Find the difference between the sum of the squares of the first
// one hundred natural numbers and the square of the sum.
// problems/euler_problem_002.rs
pub fn solve() {
    println!("Difference between the sum of the squares of the first one hundred natural numbers and the square of the sum: \n{}", difference_sum_of_squares_square_of_sums(100));
}

pub fn difference_sum_of_squares_square_of_sums(n: u32) -> u32 {
    square_of_sums(n) - sum_of_squares(n)
}

pub fn sum_of_squares(n: u32) -> u32 {
    (1..n + 1).map(|x| x * x).sum()
}

pub fn square_of_sums(n: u32) -> u32 {
    let sum: u32 = (1..n + 1).sum();
    sum * sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_squares() {
        assert_eq!(sum_of_squares(10), 385);
    }

    #[test]
    fn test_square_of_sums() {
        assert_eq!(square_of_sums(10), 3025);
    }

    #[test]
    fn test_problem_006(){
        assert_eq!(difference_sum_of_squares_square_of_sums(100), 25164150);
    }
}
