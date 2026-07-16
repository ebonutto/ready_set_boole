// Inverse (Morton code or Z-order curve)
pub fn reverse_map(n: f64) -> (u16, u16) {
    assert!(
        (0.0..=1.0).contains(&n),
        "reverse_map: map must be between 0 and 1 ({})",
        n
    );

    let z = (n * u32::MAX as f64) as u32;

    let mut x: u16 = 0;
    let mut y: u16 = 0;

    for i in 0..16 {
        y |= (((z >> (2 * i + 1)) & 1) as u16) << i;
        x |= (((z >> (2 * i)) & 1) as u16) << i;
    }

    (x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extremes() {
        assert_eq!(reverse_map(0.0), (0, 0));
        assert_eq!(reverse_map(1.0), (u16::MAX, u16::MAX));
    }

    #[test]
    fn test_single_bits() {
        assert_eq!(reverse_map(1.0 / u32::MAX as f64), (1, 0));
        assert_eq!(reverse_map(2.0 / u32::MAX as f64), (0, 1));
        assert_eq!(reverse_map(3.0 / u32::MAX as f64), (1, 1));
    }

    #[test]
    fn test_first_shift() {
        assert_eq!(reverse_map(4.0 / u32::MAX as f64), (2, 0));
        assert_eq!(reverse_map(8.0 / u32::MAX as f64), (0, 2));
    }
}
