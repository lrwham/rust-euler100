// The prime factors of 13195 are 5, 7, 13 and29
// What is the largest prime factor of the number 600851475143?
//
//

pub fn print_solve() {
    println!("Largest prime factor of the number 600851475143: {}", largest_prime_factor_result(600851475143).unwrap());
}

fn is_prime(n: u64) -> bool {
    if n == 2 {
        return true;
    }
    if n < 2 || n % 2 == 0 {
        return false;
    }
    let mut i = 3;
    let limit = (n as f64).sqrt() as u64 + 1;

    while i < limit {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true

}

pub fn largest_prime_factor_result(n: u64) -> Result<u64, &'static str> {
    if n < 2 {
        return Err("The number must be greater than 1.");
    }

    let n = largest_prime_factor(n);

    Ok(n)
}

pub fn largest_prime_factor(n: u64) -> u64 {
    throw_if_less_than_2(n);

    // start with the square root of n
    let mut factor = (n as f64).sqrt() as u64 + 1;

    while factor > 2 {
        if n % factor == 0 && is_prime(factor) {
            return factor;
        }
        factor -= 1;
    }

    return n;
}

fn throw_if_less_than_2(p0: u64) {
    if p0 < 2 {
        panic!("The number must be greater than 1.");
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    pub fn test_largest_prime_factor(){
        assert_eq!(largest_prime_factor(13195), 29);
        assert_eq!(largest_prime_factor(600851475143), 6857);
        assert_eq!(largest_prime_factor(2), 2);
    }

    #[test]
    pub fn test_throw_if_less_than_2(){
        let result = std::panic::catch_unwind(|| throw_if_less_than_2(1));
        assert!(result.is_err());

        let result = std::panic::catch_unwind(|| largest_prime_factor(1));
        assert!(result.is_err());
    }
}