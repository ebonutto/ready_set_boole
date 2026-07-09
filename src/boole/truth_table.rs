use super::boolean_evaluation::eval_formula;

pub fn print_truth_table(formula: &str) {
    // Extract unique variables, in alphabetical order
    let mut variables: Vec<char> = formula
        .chars()
        .filter(|c| c.is_ascii_uppercase())
        .collect();

    if variables.is_empty() {
        panic!("print_truth_table: invalid formula: no variables found");
    }

    variables.sort();
    variables.dedup();

    let n = variables.len();

    // Print header
    for v in &variables {
        print!("| {} ", v);
    }
    println!("| = |");

    // Print separator line
    for _ in 0..=n {
        print!("|---");
    }
    println!("|");

    // Iterate over all 2^n combinations
    for i in 0..(1u32 << n) {
        // Build the substituted formula for this combination
        let mut substituted = String::with_capacity(formula.len());
        for c in formula.chars() {
            if let Some(pos) = variables.iter().position(|&v| v == c) {
                let bit = (i >> (n - 1 - pos)) & 1;
                substituted.push(if bit == 1 { '1' } else { '0' });
            } else {
                substituted.push(c);
            }
        }

        let result = eval_formula(&substituted);

        // Print row
        for pos in 0..n {
            let bit = (i >> (n - 1 - pos)) & 1;
            print!("| {} ", bit);
        }
        println!("| {} |", if result { 1 } else { 0 });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runs_without_panic() {
        print_truth_table("AB&C|");
    }

    #[test]
    #[should_panic]
    fn test_no_variables() {
        print_truth_table("10&"); // only constants, no variables
    }
}
