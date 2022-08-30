fn sqrt_u128(number: u128) -> u128 {
    for i in 1..=number {
        if i * i > number {
            return i - 1;
        }
    }
    1
}

pub fn is_prime(number: u128) -> bool {
    let mut i = 2;
    if number % 2 == 0 {
        return number == 2;
    }
    i += 1;
    let number_sqrt = sqrt_u128(number);
    while i <= number_sqrt {
        if number % i == 0 {
            return false;
        }
        i += 2;
    }
    true
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
        assert_eq!(is_prime(5), true);
    }
    #[test]
    fn is_prime_2() {
        assert_eq!(is_prime(2), true);
    }
    #[test]
    fn is_prime_4() {
        assert_eq!(is_prime(4), false);
    }
    #[test]
    fn prime_numbers_between_2_1000() {
        for n in 2..=1000 {
            assert_eq!(is_prime(n), test_is_prime(n));
        }
    }
    #[test]
    fn is_prime_random_number() {
        let mut rng = rand::thread_rng();
        let number = rng.gen();
        assert_eq!(is_prime(number), test_is_prime(number));
    }
    #[test]
    fn is_prime_random_numbers_10() {
        let mut rng = rand::thread_rng();
        for _ in 0..=10 {
            let number = rng.gen();
            assert_eq!(is_prime(number), test_is_prime(number));
        }
    }
    #[test]
    fn sqrt_u128_9() {
        assert_eq!(sqrt_u128(9), 3);
    }
    #[test]
    fn sqrt_u128_10() {
        assert_eq!(sqrt_u128(10), 3);
    }
    #[test]
    fn sqrt_u128_perfect_to_100() {
        for i in 1..100 {
            assert_eq!(i, sqrt_u128(i * i));
        }
    }
}
