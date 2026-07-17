use std::collections::HashSet;

pub fn eval_set(formula: &str, sets: Vec<Vec<i32>>) -> Vec<i32> {
    // Compute the universal set: union of all given sets.
    let universe: HashSet<i32> = sets.iter().flatten().copied().collect();

    // Convert each input set into a HashSet for efficient operations.
    let sets: Vec<HashSet<i32>> = sets.into_iter().map(|s| s.into_iter().collect()).collect();

    let mut stack: Vec<HashSet<i32>> = Vec::new();

    for c in formula.chars() {
        match c {
            'A'..='Z' => {
                let idx = (c as u8 - b'A') as usize;
                let set = sets
                    .get(idx)
                    .unwrap_or_else(|| {
                        panic!("eval_set: not enough sets provided for variable '{}'", c)
                    })
                    .clone();
                stack.push(set);
            }
            '!' => {
                let a = stack.pop().expect("eval_set: stack underflow on '!'");
                let complement: HashSet<i32> = universe.difference(&a).copied().collect();
                stack.push(complement);
            }
            '&' => {
                let b = stack.pop().expect("eval_set: stack underflow on '&'");
                let a = stack.pop().expect("eval_set: stack underflow on '&'");
                stack.push(a.intersection(&b).copied().collect());
            }
            '|' => {
                let b = stack.pop().expect("eval_set: stack underflow on '|'");
                let a = stack.pop().expect("eval_set: stack underflow on '|'");
                stack.push(a.union(&b).copied().collect());
            }
            '^' => {
                let b = stack.pop().expect("eval_set: stack underflow on '^'");
                let a = stack.pop().expect("eval_set: stack underflow on '^'");
                stack.push(a.symmetric_difference(&b).copied().collect());
            }
            '>' => {
                // a > b  <=>  !a | b
                let b = stack.pop().expect("eval_set: stack underflow on '>'");
                let a = stack.pop().expect("eval_set: stack underflow on '>'");
                let not_a: HashSet<i32> = universe.difference(&a).copied().collect();
                stack.push(not_a.union(&b).copied().collect());
            }
            '=' => {
                // a = b  <=>  same elements (symmetric difference is empty
                // would mean equal, but we want the SET semantics: (a&b)|(!a&!b))
                let b = stack.pop().expect("eval_set: stack underflow on '='");
                let a = stack.pop().expect("eval_set: stack underflow on '='");
                let not_a: HashSet<i32> = universe.difference(&a).copied().collect();
                let not_b: HashSet<i32> = universe.difference(&b).copied().collect();
                let left: HashSet<i32> = a.intersection(&b).copied().collect();
                let right: HashSet<i32> = not_a.intersection(&not_b).copied().collect();
                stack.push(left.union(&right).copied().collect());
            }
            _ => panic!("eval_set: invalid character '{}'", c),
        }
    }

    if stack.len() != 1 {
        panic!(
            "eval_set: invalid formula: final stack has {} element(s) instead of 1",
            stack.len()
        );
    }

    stack.pop().unwrap().into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sorted(mut v: Vec<i32>) -> Vec<i32> {
        v.sort();
        v
    }

    #[test]
    fn test_subject_examples() {
        assert_eq!(
            sorted(eval_set("AB&", vec![vec![0, 1, 2], vec![0, 3, 4]])),
            vec![0]
        );
        assert_eq!(
            sorted(eval_set("AB|", vec![vec![0, 1, 2], vec![3, 4, 5]])),
            vec![0, 1, 2, 3, 4, 5]
        );
        assert_eq!(eval_set("A!", vec![vec![0, 1, 2]]), Vec::<i32>::new());
    }
}
