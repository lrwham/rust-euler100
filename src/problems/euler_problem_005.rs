// 2560 is the smallest number that can be divided by each of the numbers
// from 1 to 10 without any remainder.
// What is the smallest positive number that is evenly divisible by all
// of the numbers from 1 to 20?
pub fn print_solve() {
    println!("Slow Solution");
    println!("answer = {}", smallest_divisible_by_range(20));
    println!("Fast Solution");
    println!("answer = {}", smallest_divisible_by_range_chatgpt(20));

}

fn smallest_divisible_by_range(range_max: u32) -> u32 {
    let mut i = 1;
    loop {
        if divisible_by_range(i, range_max) {
            return i;
        }
        i += 1;
    }
}

fn divisible_by_range(numerator: u32, range_max: u32) -> bool {
    for i in 1..range_max + 1 {
        if numerator % i != 0 {
            return false;
        }
    }
    true
}

/// Provided by ChatGPT
fn smallest_divisible_by_range_chatgpt(range_max: u32) -> u32 {
    let mut lcm: u64 = 1;
    for i in 1..=range_max as u64{
        lcm = lcm * i as u64 / gcd(lcm, i);
    }
    lcm as u32
}

/// Provided by ChatGPT
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp: u64 = b;
        b = a % b;
        a = temp;
    }
    a
}

// The rest of the code remains the same


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divisble_by_range() {
        assert!(divisible_by_range(2520, 10));
    }

    #[test]
    fn test_divisble_by_range_chatgpt(){
        assert_eq!(smallest_divisible_by_range_chatgpt(10),2520);
    }

    #[test]
    fn test_fast_vs_slow(){
        assert_eq!(smallest_divisible_by_range(10), smallest_divisible_by_range_chatgpt(10));
    }
}
