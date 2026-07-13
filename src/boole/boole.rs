#[derive(Debug, Clone)]
pub enum Formula {
    Var(char),
    Not(Box<Formula>),
    And(Box<Formula>, Box<Formula>),
    Or(Box<Formula>, Box<Formula>),
    Xor(Box<Formula>, Box<Formula>),
    Implies(Box<Formula>, Box<Formula>),
    Equiv(Box<Formula>, Box<Formula>),
}

pub fn parse(formula: &str) -> Formula {
    let mut stack: Vec<Formula> = Vec::new();

    for c in formula.chars() {
        match c {
            'A'..'Z' => stack.push(Formula::Var(c)),
            '!' => {
                let a = stack.pop().expect("parse: stack underflow on '!'");
                stack.push(Formula::Not(Box::new(a)));
            }
            '&' => {
                let b = stack.pop().expect("parse: stack underflow on '&'");
                let a = stack.pop().expect("parse: stack underflow on '&'");
                stack.push(Formula::And(Box::new(a), Box::new(b)));
            }
            '|' => {
                let b = stack.pop().expect("parse: stack underflow on '|'");
                let a = stack.pop().expect("parse: stack underflow on '|'");
                stack.push(Formula::Or(Box::new(a), Box::new(b)));
            }
            '^' => {
                let b = stack.pop().expect("parse: stack underflow on '^'");
                let a = stack.pop().expect("parse: stack underflow on '^'");
                stack.push(Formula::Xor(Box::new(a), Box::new(b)));
            }
            '>' => {
                let b = stack.pop().expect("parse: stack underflow on '>'");
                let a = stack.pop().expect("parse: stack underflow on '>'");
                stack.push(Formula::Implies(Box::new(a), Box::new(b)));
            }
            '=' => {
                let b = stack.pop().expect("parse: stack underflow on '='");
                let a = stack.pop().expect("parse: stack underflow on '='");
                stack.push(Formula::Equiv(Box::new(a), Box::new(b)));
            }
            _ => panic!("parse: invalid character '{}'", c),
        }
    }

    if stack.len() != 1 {
        panic!(
            "parse: final stack has {} element(s) instead of 1",
            stack.len()
        );
    }
    stack.pop().unwrap()
}

pub fn to_rpn(f: &Formula) -> String {
    match f {
        Formula::Var(c) => c.to_string(),
        Formula::Not(a) => format!("{}!", to_rpn(a)),
        Formula::And(a, b) => format!("{}{}&", to_rpn(a), to_rpn(b)),
        Formula::Or(a, b) => format!("{}{}|", to_rpn(a), to_rpn(b)),
        Formula::Xor(a, b) => format!("{}{}^", to_rpn(a), to_rpn(b)),
        Formula::Implies(a, b) => format!("{}{}>", to_rpn(a), to_rpn(b)),
        Formula::Equiv(a, b) => format!("{}{}=", to_rpn(a), to_rpn(b)),
    }
}
