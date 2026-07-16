// Morton code or Z-order curve
pub fn map(x: u16, y: u16) -> f64 {
    let mut z: u32 = 0;

    for i in 0..16 {
        z |= (((x >> i) & 1) as u32) << (2 * i);
        z |= (((y >> i) & 1) as u32) << (2 * i + 1);
    }

    z as f64 / u32::MAX as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extremes() {
        assert_eq!(map(0, 0), 0.0);
        assert_eq!(map(u16::MAX, u16::MAX), 1.0);
    }

    #[test]
    fn test_single_bits() {
        assert_eq!(map(1, 0), 1.0 / u32::MAX as f64);
        assert_eq!(map(0, 1), 2.0 / u32::MAX as f64);
        assert_eq!(map(1, 1), 3.0 / u32::MAX as f64);
    }

    #[test]
    fn test_first_shift() {
        assert_eq!(map(2, 0), 4.0 / u32::MAX as f64);
        assert_eq!(map(0, 2), 8.0 / u32::MAX as f64);
    }

    #[test]
    fn test_in_range() {
        let n = map(12345, 54321);
        assert!(n >= 0.0);
        assert!(n <= 1.0);
    }
}
