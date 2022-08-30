use log::trace;
use num_bigint::BigUint;
use num_traits::{One, Zero};
use rand::rngs::ThreadRng;
use rand::Rng;

const MAX_TRIES_FERMAT_TEST: usize = 100;

fn mod_pow(mut base: BigUint, mut exp: BigUint, modulus: BigUint) -> BigUint {
    if modulus == One::one() {
        return Zero::zero();
    }
    let mut result: BigUint = One::one();
    base %= &modulus;
    while exp > Zero::zero() {
        if &exp % 2_u8 == One::one() {
            result = &result * &base % &modulus;
        }
        exp = &exp >> 1_u8;
        base = &base.pow(2) % &modulus;
    }
    result
}

fn is_prime(number: u128, rng: &mut ThreadRng) -> bool {
    for _ in 0..MAX_TRIES_FERMAT_TEST {
        if number == 2 || number == 3 || number == 5 {
            return true;
        }
        let r: u128 = mod_pow(
            rng.gen_range(2..number).into(),
            (number - 1).into(),
            number.into(),
        )
        .try_into()
        .unwrap();
        if r != 1 {
            return false;
        }
    }
    true
}

pub fn find_prime_factors(mut number: u128) -> Vec<u128> {
    let mut i = 2;
    let mut changed = true;
    let mut factors = Vec::new();
    let mut rng = rand::thread_rng();
    factors.push(1);
    while i <= number {
        if (changed && is_prime(number, &mut rng)) || i == number || i * 2 > number {
            trace!("{}", number);
            factors.push(number);
            break;
        } else if number % i == 0 && is_prime(i, &mut rng) {
            trace!("{}", i);
            factors.push(i);
            number /= i;

            changed = true;
            i = 2;
            continue;
        }
        i += 1;
        changed = false;
    }
    factors
}

#[cfg(test)]
mod prime_tests {
    use super::*;
    use rand::Rng;

    fn test_is_prime(n: u128) -> bool {
        for val in 2..n {
            if n % val == 0 {
                return false;
            }
        }
        true
    }

    #[test]
    fn is_prime_5() {
        let mut rng = rand::thread_rng();
        assert_eq!(is_prime(5, &mut rng), true);
    }
    #[test]
    fn is_prime_2() {
        let mut rng = rand::thread_rng();
        assert_eq!(is_prime(2, &mut rng), true);
    }
    #[test]
    fn is_prime_4() {
        let mut rng = rand::thread_rng();
        assert_eq!(is_prime(4, &mut rng), false);
    }
    #[test]
    fn prime_numbers_between_2_1000() {
        let mut rng = rand::thread_rng();
        for n in 2..=1000 {
            assert_eq!(is_prime(n, &mut rng), test_is_prime(n));
        }
    }
    #[test]
    fn is_prime_random_number() {
        let mut rng = rand::thread_rng();
        let number = rng.gen();
        assert_eq!(is_prime(number, &mut rng), test_is_prime(number));
    }
    #[test]
    fn is_prime_random_numbers_10() {
        let mut rng = rand::thread_rng();
        for _ in 0..=10 {
            let number = rng.gen();
            assert_eq!(is_prime(number, &mut rng), test_is_prime(number));
        }
    }
}
