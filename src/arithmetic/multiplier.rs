use super::adder::adder;

pub fn multiplier(mut a: u32, mut b: u32) -> u32 {
    let mut result: u32 = 0;

    while b != 0 {
        if (b & 1) == 1 {
            result = adder(result, a);
        }
        a <<= 1;
        b >>= 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_multiplication() {
        assert_eq!(multiplier(3, 5), 15);
        assert_eq!(multiplier(5, 3), 15);
        assert_eq!(multiplier(6, 4), 24);
        assert_eq!(multiplier(7, 7), 49);
    }

    #[test]
    fn multiply_by_zero() {
        assert_eq!(multiplier(0, 0), 0);
        assert_eq!(multiplier(0, 42), 0);
        assert_eq!(multiplier(42, 0), 0);
        assert_eq!(multiplier(1, 0), 0);
        assert_eq!(multiplier(0, 1), 0);
        assert_eq!(multiplier(0, u32::MAX), 0);
        assert_eq!(multiplier(u32::MAX, 0), 0);
    }

    #[test]
    fn multiply_by_one() {
        assert_eq!(multiplier(1, 1), 1);
        assert_eq!(multiplier(1, 42), 42);
        assert_eq!(multiplier(42, 1), 42);
        assert_eq!(multiplier(1, u32::MAX), u32::MAX);
        assert_eq!(multiplier(u32::MAX, 1), u32::MAX);
    }

    #[test]
    fn large_values() {
        assert_eq!(multiplier(1000, 1000), 1_000_000);
        assert_eq!(multiplier(65535, 2), 131_070);
        assert_eq!(multiplier(12357, 4097), 50_626_629);
        assert_eq!(multiplier(470496, 37), 17_408_352);
    }

    #[test]
    fn multiply_by_powers_of_two() {
        assert_eq!(multiplier(7, 2), 14);
        assert_eq!(multiplier(7, 4), 28);
        assert_eq!(multiplier(7, 8), 56);
        assert_eq!(multiplier(4, 1024), 4096);
    }

    #[test]
    fn commutativity() {
        for a in 0u32..1000 {
            for b in 0u32..1000 {
                assert_eq!(multiplier(a, b), multiplier(b, a));
            }
        }
    }

    #[test]
    fn overflow() {
        assert_eq!(multiplier(u32::MAX, 2), u32::MAX.wrapping_mul(2));
        assert_eq!(
            multiplier(u32::MAX, u32::MAX),
            u32::MAX.wrapping_mul(u32::MAX)
        );
        assert_eq!(multiplier(1 << 31, 2), 0);
    }

    #[test]
    fn matches_wrapping_mul() {
        for a in 0u32..1000 {
            for b in 0u32..1000 {
                assert_eq!(multiplier(a, b), a.wrapping_mul(b));
            }
        }
    }
}
