use super::boole::{parse, to_nnf, to_rpn};

pub fn negation_normal_form(formula: &str) -> String {
    let ast = parse(formula);
    let nnf = to_nnf(ast);
    to_rpn(&nnf)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(negation_normal_form("A"), "A");
        assert_eq!(negation_normal_form("A!"), "A!");
        assert_eq!(negation_normal_form("AB&!"), "A!B!|");
        assert_eq!(negation_normal_form("AB|!"), "A!B!&");
    }

    #[test]
    fn operators() {
        assert_eq!(negation_normal_form("AB^"), "AB!&A!B&|");
        assert_eq!(negation_normal_form("AB>"), "A!B|");
        assert_eq!(negation_normal_form("AB="), "AB&A!B!&|");
    }

    #[test]
    fn negations() {
        assert_eq!(negation_normal_form("A!!"), "A");
        assert_eq!(negation_normal_form("AB^!"), "AB&A!B!&|");
        assert_eq!(negation_normal_form("AB>!"), "AB!&");
        assert_eq!(negation_normal_form("AB=!"), "AB!&A!B&|");
    }

    #[test]
    fn complex() {
        assert_eq!(negation_normal_form("AB|C&!"), "A!B!&C!|");
        assert_eq!(negation_normal_form("AB&C|!"), "A!B!|C!&");
    }
}
