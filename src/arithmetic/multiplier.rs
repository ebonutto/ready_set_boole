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
    fn basic() {
        assert_eq!(multiplier(1, 2), 2);
        assert_eq!(multiplier(6, 7), 42);
        assert_eq!(multiplier(14, 37), 518);
    }

    #[test]
    fn zero() {
        assert_eq!(multiplier(1, 0), 0);
        assert_eq!(multiplier(0, 1), 0);
        assert_eq!(multiplier(0, 0), 0);
    }

    #[test]
    fn one() {
        assert_eq!(multiplier(42, 1), 42);
        assert_eq!(multiplier(1, 42), 42);
        assert_eq!(multiplier(1, 1), 1);
    }

    #[test]
    fn large() {
        assert_eq!(multiplier(1_000, 1_000), 1_000_000);
        assert_eq!(multiplier(65_535, 2), 131_070);
        assert_eq!(multiplier(12_357, 4_097), 50_626_629);
    }

    #[test]
    fn powers_of_two() {
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
