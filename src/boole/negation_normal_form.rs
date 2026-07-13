use super::boole::{Formula, parse, to_nnf, to_rpn};

pub fn negation_normal_form(formula: &str) -> String {
    let ast = parse(formula);
    let nnf_ast = to_nnf(ast);
    to_rpn(&nnf_ast)
}
