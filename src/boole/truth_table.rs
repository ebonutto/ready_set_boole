use super::boolean_evaluation::eval_formula;

pub fn print_truth_table(formula: &str) {
    let mut variables: Vec<char> = formula.chars().filter(|c| c.is_ascii_uppercase()).collect();

    if variables.is_empty() {
        panic!("print_truth_table: invalid formula: no variable found");
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

        let result = eval_formula(&substituted);

        // Print row
        for pos in 0..n {
            let bit = (mask >> (n - 1 - pos)) & 1;
            print!("| {} ", bit);
        }
        println!("| {} |", if result { 1 } else { 0 });
    }
}
