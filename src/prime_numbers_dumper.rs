use std::{
    fs::File,
    io::Write,
    time::Duration,
    thread,
};


/// Iterate and get prime numbers infinitely. On every prime number, append it to the file.
pub fn run_prime_numbers_dumper() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "target/found_primes.txt";
    println!("Starting to find and append prime numbers to '{}'...", file_path);
    println!("Press Ctrl+C to stop.");

    let mut file = File::options()
        .create(true)
        .append(true)
        .open(file_path)?;

    let mut current_number: u128 = 2;

    loop {
        if is_prime(current_number) {
            writeln!(file, "{}", current_number)?;
            println!("Found and wrote prime: {}", current_number);
        }

        current_number += 1;
        thread::sleep(Duration::from_millis(1));
    }
}


fn is_prime(number: u128) -> bool {
    if number <= 1 { return false }
    if number == 2 { return true }
    if number % 2 == 0 { return false }

    let number_squared = number.isqrt();
    for i in (3..=number_squared).step_by(2) {
        if number % i == 0 {return false }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(4));
        assert!(is_prime(5));
        assert!(!is_prime(6));
        assert!(is_prime(7));
        assert!(!is_prime(9)); // 3*3
        assert!(!is_prime(25)); // 5*5
        assert!(!is_prime(49)); // 7*7
        assert!(is_prime(17));
        assert!(is_prime(97));
        assert!(!is_prime(100));
        assert!(!is_prime(99)); // 9*11
        assert!(is_prime(2_147_483_647)); // A large prime (Mersenne prime)
    }
}