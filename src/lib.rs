#![allow(unused_variables, dead_code, unused_mut, unused_imports)]

fn is_prime(n: u32, primes: &Vec<u32>) -> bool {
    if n <= 1 {
        return false;
    }
    primes.iter().all(|prime| n % prime != 0)
}

pub fn nth(n: u32) -> u32 {
    let mut primes = Vec::with_capacity(n.max(2) as usize);
    primes.push(2);
    primes.push(3);

    if n < 200 {
        fn find_primes(count: u32, step: u32, primes: &mut Vec<u32>) {
            if primes.len() < (count + 1) as usize {
                if is_prime(step, primes) {
                    primes.push(step);
                }
                find_primes(count, step + 2, primes); // iterate by 2, since all primes after 2 are odd.
            }
        }

        find_primes(n, 3, &mut primes); // starting at 2 will loop forever.
    } else {
        while primes.len() < (n + 1) as usize {
            let mut next_guess = primes.last().expect("primes vector is empty.") + 2;
            while primes.iter().any(|prime| next_guess % prime == 0) {
                next_guess += 2; // iterate by 2, since all primes after 2 are odd.
            }
            primes.push(next_guess);
        }
    }
    primes[n as usize]
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
}
