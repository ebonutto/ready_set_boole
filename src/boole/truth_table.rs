use super::boolean_evaluation::eval_formula;

fn print_header(variables: &[char]) {
    for v in variables {
        print!("| {} ", v);
    }
    println!("| = |");
}

fn print_separator_line(n: usize) {
    for _ in 0..=n {
        print!("|---");
    }
    println!("|");
}

fn print_row(n: usize, mask: usize, result: bool) {
    for pos in 0..n {
        let bit = (mask >> (n - 1 - pos)) & 1;
        print!("| {} ", bit);
    }
    println!("| {} |", if result { 1 } else { 0 });
}

pub fn print_truth_table(formula: &str) {
    let mut variables: Vec<char> = formula.chars().filter(|c| c.is_ascii_uppercase()).collect();

    if variables.is_empty() {
        panic!("print_truth_table: invalid formula: no variable found");
    }

    variables.sort();
    variables.dedup();

    let n = variables.len();

    print_header(&variables);
    print_separator_line(n);

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

        print_row(n, mask, result);
    }
}
