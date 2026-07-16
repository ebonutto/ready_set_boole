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
        assert_eq!(eval_formula("10&"), false); // 1 & 0
        assert_eq!(eval_formula("10|"), true); // 1 | 0
        assert_eq!(eval_formula("11>"), true); // 1 > 1
        assert_eq!(eval_formula("10="), false); // 1 = 0
        assert_eq!(eval_formula("1011||="), true); // 1 = ((1 | 1) | 0)
    }

    #[test]
    fn test_not() {
        assert_eq!(eval_formula("1!"), false);
        assert_eq!(eval_formula("0!"), true);
        assert_eq!(eval_formula("0!!"), false);
    }

    #[test]
    fn test_complex_formulas() {
        assert_eq!(eval_formula("11&0|"), true); // (1 && 1) || 0
        assert_eq!(eval_formula("10|0&"), false); // (1 || 0) && 0
        assert_eq!(eval_formula("10>"), false); // 1 > 0
        assert_eq!(eval_formula("01>"), true); // 0 > 1
        assert_eq!(eval_formula("00="), true); // 0 = 0
        assert_eq!(eval_formula("11="), true); // 1 = 1
    }

    #[test]
    #[should_panic]
    fn test_empty_formula() {
        eval_formula("");
    }

    #[test]
    #[should_panic]
    fn test_operator_without_operands() {
        eval_formula("&");
    }

    #[test]
    #[should_panic]
    fn test_only_not() {
        eval_formula("!");
    }

    #[test]
    #[should_panic]
    fn test_too_many_operators() {
        eval_formula("11&&");
    }

    #[test]
    #[should_panic]
    fn test_leftover_values() {
        eval_formula("11");
    }

    #[test]
    #[should_panic]
    fn test_invalid_character() {
        eval_formula("A");
    }

    #[test]
    #[should_panic]
    fn test_stack_underflow() {
        eval_formula("1&");
    }
}
