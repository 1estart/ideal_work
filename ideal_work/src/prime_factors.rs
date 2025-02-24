fn prime_factors(n: u32) -> Vec<u32> {
    let mut factors = Vec::new();
    let mut n = n;

    for divisor in 2..=n {
        if n <= 1 {
            break;
        }

        for _ in 0.. {
            if n % divisor != 0 {
                break;
            }
            factors.push(divisor);
            n /= divisor;
        }
    }

    factors
}

fn contains_subvec<T: PartialEq>(vec: &[T], subvec: &[T]) -> bool {
    if subvec.len() > vec.len() {
        return false;
    }

    vec.windows(subvec.len()).any(|window| window == subvec)
}

#[cfg(test)]
mod prime_factors_tests {
    use super::*;

    #[test]
    fn test_prime_factors_of_1() {
        assert_eq!(prime_factors(1), vec![]);
    }
    #[test]
    fn test_prime_factors_of_2() {
        assert_eq!(prime_factors(2), Vec::from([2]));
    }

    #[test]
    fn test_prime_factors_of_3() {
        assert!(prime_factors(3).contains(&3));
    }

    #[test]
    fn factors_4_is_2_2() {
        assert!(contains_subvec(&prime_factors(4), &[2, 2]))
    }

    #[test]
    fn factors_5() {
        assert_eq!(prime_factors(5), Vec::from([5]))
    }

    #[test]
    fn factors_6() {
        assert_eq!(prime_factors(6), Vec::from([2, 3]))
    }

    #[test]
    fn factors_7() {
        assert_eq!(prime_factors(7), Vec::from([7]))
    }

    #[test]
    fn factors_8() {
        assert_eq!(prime_factors(8), Vec::from([2, 2, 2]))
    }

    #[test]
    fn factors_9() {
        assert_eq!(prime_factors(9), Vec::from([3, 3]))
    }

    #[test]
    fn factors_145() {
        assert_eq!(prime_factors(145), Vec::from([5, 29]))
    }
}
