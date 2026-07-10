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
