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

pub fn to_nnf(f: Formula) -> Formula {
    match f {
        // A variable is already in NNF, negated or not
        Formula::Var(_) => f,

        Formula::Not(inner) => match *inner {
            // A negated variable is already NNF
            Formula::Var(c) => Formula::Not(Box::new(Formula::Var(c))),

            // !!A => A
            Formula::Not(a) => to_nnf(*a),

            // !(A & B) => !A | !B
            Formula::And(a, b) => Formula::Or(
                Box::new(to_nnf(Formula::Not(a))),
                Box::new(to_nnf(Formula::Not(b))),
            ),

            // !(A | B) => !A & !B
            Formula::Or(a, b) => Formula::And(
                Box::new(to_nnf(Formula::Not(a))),
                Box::new(to_nnf(Formula::Not(b))),
            ),

            // !(A ^ B) => A = B
            Formula::Xor(a, b) => to_nnf(Formula::Equiv(a, b)),

            // !(A > B) => A & !B
            Formula::Implies(a, b) => {
                Formula::And(Box::new(to_nnf(*a)), Box::new(to_nnf(Formula::Not(b))))
            }

            // !(A = B) => A ^ B
            Formula::Equiv(a, b) => to_nnf(Formula::Xor(a, b)),
        },

        // A & B => recurse on both sides
        Formula::And(a, b) => Formula::And(Box::new(to_nnf(*a)), Box::new(to_nnf(*b))),

        // A | B => recurse on both sides
        Formula::Or(a, b) => Formula::Or(Box::new(to_nnf(*a)), Box::new(to_nnf(*b))),

        // (A ^ B) => (A & !B) | (!A & B)
        Formula::Xor(a, b) => {
            let left = Formula::And(a.clone(), Box::new(Formula::Not(b.clone())));
            let right = Formula::And(Box::new(Formula::Not(a)), b);
            to_nnf(Formula::Or(Box::new(left), Box::new(right)))
        }

        // (A > B) => !A | B, then recurse
        Formula::Implies(a, b) => {
            to_nnf(Formula::Or(Box::new(Formula::Not(a)), Box::new(to_nnf(*b))))
        }

        // (A = B) => (A & B) | (!A & !B), then recurse
        Formula::Equiv(a, b) => {
            let left = Formula::And(a.clone(), b.clone());
            let right = Formula::And(Box::new(Formula::Not(a)), Box::new(Formula::Not(b)));
            to_nnf(Formula::Or(Box::new(left), Box::new(right)))
        }
    }
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
