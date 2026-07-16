use super::boole::{parse, to_cnf, to_nnf, to_rpn};

pub fn conjunctive_normal_form(formula: &str) -> String {
    let ast = parse(formula);
    let nnf = to_nnf(ast);
    let cnf = to_cnf(nnf);
    to_rpn(&cnf)
}
