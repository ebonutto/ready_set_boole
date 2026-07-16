pub fn adder(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let carry = (a & b) << 1;
        a ^= b;
        b = carry
    }

    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
        assert_eq!(adder(0, 0), 0);
        assert_eq!(adder(0, 5), 5);
        assert_eq!(adder(5, 0), 5);
    }

    #[test]
    fn test_basic() {
        assert_eq!(adder(1, 1), 2);
        assert_eq!(adder(3, 4), 7);
    }

    #[test]
    fn test_carry_cascade() {
        assert_eq!(adder(0b0111, 0b0001), 0b1000);
    }

    #[test]
    fn test_overflow_wraparound() {
        assert_eq!(adder(u32::MAX, 1), 0);
        assert_eq!(adder(u32::MAX, u32::MAX), u32::MAX.wrapping_add(u32::MAX));
    }

    #[test]
    fn test_against_native_add() {
        for a in 0u32..1000 {
            for b in 0u32..1000 {
                assert_eq!(adder(a, b), a.wrapping_add(b));
            }
        }
    }
}
