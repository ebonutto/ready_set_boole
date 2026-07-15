pub fn eval_formula(formula: &str) -> bool {
    // Evaluates the given propositional formula in RPN.

    let mut stack: Vec<bool> = Vec::new();

    for c in formula.chars() {
        match c {
            // False constant
            '0' => stack.push(false),

            // True constant
            '1' => stack.push(true),

            // Logical negation
            '!' => {
                let a: bool = stack
                    .pop()
                    .expect("eval_formula: invalid formula: stack underflow on '!'");
                stack.push(!a);
            }

            // Logical conjunction
            '&' => {
                let b: bool = stack
                    .pop()
                    .expect("eval_formula: invalid formula: stack underflow on '&'");
                let a: bool = stack
                    .pop()
                    .expect("eval_formula: invalid formula: stack underflow on '&'");
                stack.push(a & b);
            }

            // Logical disjunction
            '|' => {
                let b: bool = stack
                    .pop()
                    .expect("eval_formula: invalid formula: stack underflow on '|'");
                let a: bool = stack
                    .pop()
                    .expect("eval_formula: invalid formula: stack underflow on '|'");
                stack.push(a | b);
            }

            // Exclusive disjunction (XOR)
            '^' => {
                let b: bool = stack
                    .pop()
                    .expect("eval_formula: invalid formula: stack underflow on '^'");
                let a: bool = stack
                    .pop()
                    .expect("eval_formula: invalid formula: stack underflow on '^'");
                stack.push(a ^ b);
            }

            // Material implication
            '>' => {
                let b: bool = stack
                    .pop()
                    .expect("eval_formula: invalid formula: stack underflow on '>'");
                let a: bool = stack
                    .pop()
                    .expect("eval_formula: invalid formula: stack underflow on '>'");
                stack.push(!a | b);
            }

            // Logical equivalence
            '=' => {
                let b: bool = stack
                    .pop()
                    .expect("eval_formula: invalid formula: stack underflow on '='");
                let a: bool = stack
                    .pop()
                    .expect("eval_formula: invalid formula: stack underflow on '='");
                stack.push(a == b);
            }

            _ => panic!("eval_formula: invalid formula: invalid character '{}'", c),
        }
    }

    if stack.len() != 1 {
        panic!(
            "eval_formula: invalid formula: final stack has {} element(s) instead of 1",
            stack.len()
        );
    }
    stack.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        assert_eq!(eval_formula("10&"), false);
        assert_eq!(eval_formula("10|"), true);
        assert_eq!(eval_formula("11>"), true);
        assert_eq!(eval_formula("10="), false);
        assert_eq!(eval_formula("1011||="), true);
    }
}
