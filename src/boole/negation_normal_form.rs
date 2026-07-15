use super::boole::{parse, to_nnf, to_rpn};

pub fn negation_normal_form(formula: &str) -> String {
    // Converts the given RPN formula to its negation normal form (NNF).

    let ast = parse(formula);
    let nnf = to_nnf(ast);
    to_rpn(&nnf)
}
