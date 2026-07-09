pub fn powerset(set: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    let n = set.len();

    for mask in 0..(1u32 << n) {
        let mut subset: Vec<i32> = Vec::new();

        for i in 0..n {
            if mask & (1 << i) != 0 {
                subset.push(set[i]);
            }
        }

        result.push(subset);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::powerset;

    #[test]
    fn test_empty_set() {
        let result = powerset(vec![]);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], Vec::<i32>::new());
    }

    #[test]
    fn test_single_element() {
        let result = powerset(vec![42]);

        assert_eq!(result.len(), 2);
        assert!(result.contains(&vec![]));
        assert!(result.contains(&vec![42]));
    }

    #[test]
    fn test_two_elements() {
        let result = powerset(vec![1, 2]);

        assert_eq!(result.len(), 4);

        assert!(result.contains(&vec![]));
        assert!(result.contains(&vec![1]));
        assert!(result.contains(&vec![2]));
        assert!(result.contains(&vec![1, 2]));
    }

    #[test]
    fn test_three_elements() {
        let result = powerset(vec![1, 2, 3]);

        assert_eq!(result.len(), 8);

        let expected = vec![
            vec![],
            vec![1],
            vec![2],
            vec![3],
            vec![1, 2],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3],
        ];

        for subset in expected {
            assert!(result.contains(&subset));
        }
    }

    #[test]
    fn test_size_property() {
        let set = vec![1, 2, 3, 4, 5];

        let result = powerset(set);

        assert_eq!(result.len(), 32);
    }
}
