use super::boolean_evaluation::eval_formula;

pub fn sat(formula: &str) -> bool {
    // Check if the given RPN formula is satisfiable:
    // returns true if at least one combination of variable values makes it true.

    // Extract variables from the formula
    let mut variables: Vec<char> = formula.chars().filter(|c| c.is_ascii_uppercase()).collect();

    if variables.is_empty() {
        return eval_formula(formula);
    }

    // Sort and remove duplicate variables
    variables.sort();
    variables.dedup();

    let n = variables.len();

    // Iterate over all 2^n possible combinations
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
