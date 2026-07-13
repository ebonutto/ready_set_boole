use super::boole::{Formula, parse, to_nnf, to_rpn};

pub fn conjunctive_normal_form(formula: &str) -> String {
    ast = parse(formula);
    nnf_ast = to_nnf(ast);
    cnf_ast = to_cnf(nnf_ast);
    to_rpn(&cnf_ast)
}
