// problems/euler_problem_002.rs
pub fn print_solve() {
    println!(
        "Sum of even Fibonacci numbers below 4 million: {}",
        sum_of_even_fibonacci_numbers(4_000_000)
    );
}

/// Used to solve problem 2.
pub fn next_fibonacci_number(previous: u32, current: u32) -> u32 {
    previous + current
}

/// Used to solve problem 2.
pub fn sum_of_even_fibonacci_numbers(limit: u32) -> u32 {
    let mut previous = 1;
    let mut current = 2;
    let mut sum: u32 = 0;

    while current < limit {
        if current % 2 == 0 {
            sum += current;
        }
        let next = next_fibonacci_number(previous, current);
        previous = current;
        current = next;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_002() {
        assert_eq!(sum_of_even_fibonacci_numbers(10), 10); // 2 + 8
        assert_eq!(sum_of_even_fibonacci_numbers(90), 44); // 2 + 8 + 34
        assert_eq!(sum_of_even_fibonacci_numbers(4_000_000), 4613732);
    }
}
