pub fn adder(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let carry = (a & b) << 1;
        a ^= b;
        b = carry;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(adder(1, 1), 2);
        assert_eq!(adder(3, 4), 7);
        assert_eq!(adder(37, 63), 100);
    }

    #[test]
    fn zero() {
        assert_eq!(adder(1, 0), 1);
        assert_eq!(adder(0, 1), 1);
        assert_eq!(adder(0, 0), 0);
    }

    #[test]
    fn large() {
        assert_eq!(adder(1000, 1000), 2000);
        assert_eq!(adder(65_535, 2), 65_537);
        assert_eq!(adder(470_496, 37), 470_533);
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
