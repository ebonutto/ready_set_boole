use super::boole::{parse, to_cnf, to_nnf, to_rpn};

pub fn conjunctive_normal_form(formula: &str) -> String {
    let ast = parse(formula);
    let nnf = to_nnf(ast);
    let cnf = to_cnf(nnf);
    to_rpn(&cnf)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(conjunctive_normal_form("A"), "A");
        assert_eq!(conjunctive_normal_form("A!"), "A!");
        assert_eq!(conjunctive_normal_form("AB&!"), "A!B!|");
        assert_eq!(conjunctive_normal_form("AB|!"), "A!B!&");
        assert_eq!(conjunctive_normal_form("AB|C&"), "AB|C&");
        assert_eq!(conjunctive_normal_form("AB&C|"), "AC|BC|&");
    }

    #[test]
    fn operators() {
        assert_eq!(conjunctive_normal_form("AB^"), "AA!|AB|&B!A!|B!B|&&");
        assert_eq!(conjunctive_normal_form("AB>"), "A!B|");
        assert_eq!(conjunctive_normal_form("AB="), "AA!|AB!|&BA!|BB!|&&");
    }

    #[test]
    fn negations() {
        assert_eq!(conjunctive_normal_form("A!!"), "A");
        assert_eq!(conjunctive_normal_form("AB^!"), "AA!|AB!|&BA!|BB!|&&");
        assert_eq!(conjunctive_normal_form("AB>!"), "AB!&");
        assert_eq!(conjunctive_normal_form("AB=!"), "AA!|AB|&B!A!|B!B|&&");
    }

    #[test]
    fn complex() {
        assert_eq!(conjunctive_normal_form("AB|C&!"), "A!C!|B!C!|&");
        assert_eq!(conjunctive_normal_form("AB&C|!"), "A!B!|C!&");
        assert_eq!(conjunctive_normal_form("AB|C|D|"), "AB|C|D|");
        assert_eq!(conjunctive_normal_form("AB&C&D&"), "AB&C&D&");
        assert_eq!(conjunctive_normal_form("AB&!C!|"), "A!B!|C!|");
        assert_eq!(conjunctive_normal_form("AB|!C!&"), "A!B!&C!&");
    }
}
