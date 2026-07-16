use super::boolean_evaluation::eval_formula;

pub fn sat(formula: &str) -> bool {
    let mut variables: Vec<char> = formula.chars().filter(|c| c.is_ascii_uppercase()).collect();

    if variables.is_empty() {
        return eval_formula(formula);
    }

    variables.sort();
    variables.dedup();

    let n = variables.len();

    for mask in 0..(1usize << n) {
        let mut substituted = String::with_capacity(formula.len());

        for c in formula.chars() {
            if let Some(pos) = variables.iter().position(|&v| v == c) {
                let bit = (mask >> (n - 1 - pos)) & 1;
                substituted.push(if bit == 1 { '1' } else { '0' });
            } else {
                substituted.push(c);
            }
        }

        if eval_formula(&substituted) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subject() {
        assert_eq!(sat("AB|"), true);
        assert_eq!(sat("AB&"), true);
        assert_eq!(sat("AA!&"), false);
        assert_eq!(sat("AA^"), false);
    }
}
