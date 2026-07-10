pub fn map(x: u16, y: u16) -> f64 {
    let mut z: u32 = 0;

    for i in 0..16 {
        z |= (((x >> i) & 1) as u32) << (2 * i);
        z |= (((y >> i) & 1) as u32) << (2 * i + 1);
    }

    z as f64 / u32::MAX as f64
}
