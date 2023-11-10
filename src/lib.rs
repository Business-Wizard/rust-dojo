#![allow(unused_variables, dead_code, unused_mut, unused_imports)]

fn is_prime(n: u32, primes: &Vec<u32>) -> bool {
    if n <= 1 {
        return false;
    }
    for p in primes {
        if n % p == 0 {
            return false;
        }
    }
    true
}

pub fn nth(n: u32) -> u32 {
    let mut primes = Vec::from([2]);

    if n < 200 {
        fn find_primes(count: u32, step: u32, primes: &mut Vec<u32>) {
            if primes.len() < (count + 1) as usize {
                if is_prime(step, primes) {
                    primes.push(step);
                }
                find_primes(count, step + 1, primes);
            }
        }

        find_primes(n, 2, &mut primes);
    } else {
        let mut step = 3;
        while primes.len() < (n + 1) as usize {
            if is_prime(step, &primes) {
                primes.push(step);
            }
            step += 1;
        }
    }

    *primes.last().expect("No primes found")
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_prime() {
        assert_eq!(nth(0), 2);
    }

    #[test]
    fn second_prime() {
        assert_eq!(nth(1), 3);
    }

    #[test]
    fn sixth_prime() {
        assert_eq!(nth(5), 13);
    }

    #[test]
    fn big_prime() {
        assert_eq!(nth(10_000), 104_743);
    }

    #[test]
    fn is_prime_2() {
        assert!(is_prime(2, &vec![]));
    }

    #[test]
    fn is_prime_3() {
        assert!(is_prime(3, &vec![]));
    }
}
