pub fn powerset(set: Vec<i32>) -> Vec<Vec<i32>> {
    let n = set.len();
    assert!(
        n < usize::BITS as usize,
        "powerset: set too large, would overflow (max {} elements)",
        usize::BITS as usize
    );

    let size = 1usize << n;
    let mut result: Vec<Vec<i32>> = Vec::with_capacity(size);

    for mask in 0..size {
        let mut subset: Vec<i32> = Vec::with_capacity(mask.count_ones() as usize);

        let mut m = mask;
        while m != 0 {
            let i = m.trailing_zeros() as usize; // Index of the lowest set bit
            subset.push(set[i]);
            m &= m - 1; // Clear the lowest set bit
        }

        result.push(subset);
    }

    result
}

//#[cfg(test)]
//mod tests {
//    use super::powerset;

//}
