use crate::boole::{Formula, parse, to_rpn};

fn to_nnf(f: Formula) -> Formula {
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

pub fn negation_normal_form(formula: &str) -> String {
    let ast = parse(formula);
    let nnf_ast = to_nnf(ast);
    to_rpn(&nnf_ast)
}
