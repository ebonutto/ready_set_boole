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
    fn basic_addition() {
        assert_eq!(adder(1, 1), 2);
        assert_eq!(adder(3, 4), 7);
        assert_eq!(adder(42, 58), 100);
    }

    #[test]
    fn add_with_zero() {
        assert_eq!(adder(0, 0), 0);
        assert_eq!(adder(0, 42), 42);
        assert_eq!(adder(42, 0), 42);
        assert_eq!(adder(0, u32::MAX), u32::MAX);
        assert_eq!(adder(u32::MAX, 0), u32::MAX);
    }

    #[test]
    fn large_values() {
        assert_eq!(adder(1000, 1000), 2000);
        assert_eq!(adder(65535, 2), 65537);
        assert_eq!(adder(12357, 4097), 16454);
        assert_eq!(adder(470496, 37), 470533);
    }

    #[test]
    fn carry_propagation() {
        assert_eq!(adder(0b0111, 0b0001), 0b1000);
        assert_eq!(adder(0b1111, 0b0001), 0b1_0000);
        assert_eq!(adder(0x00FF_FFFF, 1), 0x0100_0000);
    }

    #[test]
    fn commutativity() {
        for a in 0u32..1000 {
            for b in 0u32..1000 {
                assert_eq!(adder(a, b), adder(b, a));
            }
        }
    }

    #[test]
    fn overflow() {
        assert_eq!(adder(u32::MAX, 1), 0);
        assert_eq!(adder(u32::MAX, u32::MAX), u32::MAX.wrapping_add(u32::MAX));
    }

    #[test]
    fn matches_wrapping_add() {
        for a in 0u32..1000 {
            for b in 0u32..1000 {
                assert_eq!(adder(a, b), a.wrapping_add(b));
            }
        }
    }
}
