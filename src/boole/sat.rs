use super::boolean_evaluation::eval_formula;

pub fn sat(formula: &str) -> bool {
    // Extract unique variables, in alphabetical order
    let mut variables: Vec<char> = formula
        .chars()
        .filter(|c| c.is_ascii_uppercase())
        .collect();

    if variables.is_empty() {
        return eval_formula(formula);
    }

    variables.sort();
    variables.dedup();

    let n = variables.len();

    // Iterate over all 2^n combinations
    for mask in 0..(1usize << n) {
        // Build the substituted formula for this combination
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
