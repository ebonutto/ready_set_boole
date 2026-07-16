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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(powerset(vec![]), vec![vec![]]);
    }

    #[test]
    fn test_one_element() {
        assert_eq!(powerset(vec![1]), vec![vec![], vec![1]]);
    }

    #[test]
    fn test_two_elements() {
        assert_eq!(
            powerset(vec![1, 2]),
            vec![vec![], vec![1], vec![2], vec![1, 2]]
        );
    }

    #[test]
    fn test_three_elements() {
        assert_eq!(
            powerset(vec![1, 2, 3]),
            vec![
                vec![],
                vec![1],
                vec![2],
                vec![1, 2],
                vec![3],
                vec![1, 3],
                vec![2, 3],
                vec![1, 2, 3]
            ]
        );
    }

    #[test]
    fn test_negative_numbers() {
        assert_eq!(
            powerset(vec![-1, 42]),
            vec![vec![], vec![-1], vec![42], vec![-1, 42]]
        );
    }

    #[test]
    #[should_panic]
    fn test_set_too_large() {
        powerset(vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8,
            9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7,
            8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6,
            7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
        ]);
    }
}
