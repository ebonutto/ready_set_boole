use super::boolean_evaluation::eval_formula;

pub fn sat(formula: &str) -> bool {
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

    let mut indexes: [Option<usize>; 26] = [None; 26];

    for (i, &c) in variables.iter().enumerate() {
        indexes[(c as u8 - b'A') as usize] = Some(i);
    }

    let chars: Vec<char> = formula.chars().collect();
    let combinations = 1usize << n;

    for mask in 0..combinations {
        let mut substituted = String::with_capacity(chars.len());

        for &c in &chars {
            match c {
                'A'..='Z' => {
                    let pos = indexes[(c as u8 - b'A') as usize].unwrap();
                    let bit = (mask >> (n - 1 - pos)) & 1;
                    substituted.push(if bit == 1 { '1' } else { '0' });
                }
                _ => substituted.push(c),
            }
        }

        if eval_formula(&substituted) {
            return true;
        }
    }

    false
}
