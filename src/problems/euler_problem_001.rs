// problems/euler_problem_001.rs
pub fn solve() {
    println!(
        "Sum of multiples of 3 or 5 below 1000: {}",
        sum_of_multiples(3, 5, 1000)
    );
}

/// Used to solve problem 1.
pub fn sum_of_multiples(multiple_1: u32, multiple_2: u32, limit: u32) -> u32 {
    (1..limit)
        .filter(|&x| x % multiple_1 == 0 || x % multiple_2 == 0)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_001() {
        assert_eq!(sum_of_multiples(3, 5, 4), 3);
        assert_eq!(sum_of_multiples(3, 5, 6), 8);
        assert_eq!(sum_of_multiples(3, 5, 10), 23);
        assert_eq!(sum_of_multiples(3, 5, 1000), 233168);
    }
}