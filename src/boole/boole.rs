//enum Formula {
//    Var(char),
//    Not(Box<Formula>),
//    And(Box<Formula>, Box<Formula>),
//    Or(Box<Formula>, Box<Formula>),
//    Xor(Box<Formula>, Box<Formula>),
//    Implies(Box<Formula>, Box<Formula>),
//    Equiv(Box<Formula>, Box<Formula>),
//}

//pub fn parse(formula &str) -> Formula {
//    let mut stack: Vec<Formula> = Vec::new();

//    for c in formula.chars() {
//        match c {
//            'A'..='Z' => stack.push(Formula::Var(c)),
//            '!' => {
//                let a = stack.pop().expect("parse: stack underflow on '!'");
//                stack.push(Formula::Not(Box::new(a)));
//            }
//            '&' => {

//            }
//            '|' => {

//            }
//            '^' => {

//            }
//            '>' => {

//            }
//            '=' => {

//            }
//            _=>panic!("parse: final stack has {} element(s) insted of 1", stack.len)
//        }
//    }
//}
