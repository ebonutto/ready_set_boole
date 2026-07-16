pub fn eval_formula(formula: &str) -> bool {
    let mut stack: Vec<bool> = Vec::new();

    for c in formula.chars() {
        match c {
            '0' => stack.push(false),
            '1' => stack.push(true),
            '!' => {
                let a: bool = stack
                    .pop()
                    .expect("eval_formula: invalid formula: stack underflow on '!'");
                stack.push(!a);
            }
            '&' => {
                let b: bool = stack
                    .pop()
                    .expect("eval_formula: invalid formula: stack underflow on '&'");
                let a: bool = stack
                    .pop()
                    .expect("eval_formula: invalid formula: stack underflow on '&'");
                stack.push(a & b);
            }
            '|' => {
                let b: bool = stack
                    .pop()
                    .expect("eval_formula: invalid formula: stack underflow on '|'");
                let a: bool = stack
                    .pop()
                    .expect("eval_formula: invalid formula: stack underflow on '|'");
                stack.push(a | b);
            }
            '^' => {
                let b: bool = stack
                    .pop()
                    .expect("eval_formula: invalid formula: stack underflow on '^'");
                let a: bool = stack
                    .pop()
                    .expect("eval_formula: invalid formula: stack underflow on '^'");
                stack.push(a ^ b);
            }
            '>' => {
                let b: bool = stack
                    .pop()
                    .expect("eval_formula: invalid formula: stack underflow on '>'");
                let a: bool = stack
                    .pop()
                    .expect("eval_formula: invalid formula: stack underflow on '>'");
                stack.push(!a | b);
            }
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
    fn test_subject() {
        assert_eq!(eval_formula("10&"), false);
        assert_eq!(eval_formula("10|"), true);
        assert_eq!(eval_formula("11>"), true);
        assert_eq!(eval_formula("10="), false);
        assert_eq!(eval_formula("1011||="), true);
    }

    #[test]
    #[should_panic]
    fn test_empty_formula() {
        eval_formula("");
    }

    #[test]
    #[should_panic]
    fn test_leftover_values() {
        eval_formula("11"); // Two operands, no operator: stack ends with 2 elements.
    }

    #[test]
    #[should_panic]
    fn test_invalid_character() {
        eval_formula("A"); // 'A' is not a valid RPN symbol for this exercise.
    }

    #[test]
    #[should_panic]
    fn test_stack_underflow() {
        eval_formula("1&"); // '&' needs two operands, only one is available.
    }
}
