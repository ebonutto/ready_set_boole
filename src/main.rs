// mod arithmetic;
// mod boole;
// mod gray_code;
mod set;
// mod space;

// use arithmetic::adder::adder;
// use arithmetic::multiplier::multiplier;

// use gray_code::gray_code;

// use boole::conjunctive_normal_form::conjunctive_normal_form;
// use boole::boolean_evaluation::eval_formula;
// use boole::negation_normal_form::negation_normal_form;
// use boole::truth_table::print_truth_table;

// use set::powerset::powerset;
// use set::set_evaluation::eval_set;

// use space::curve::map;

fn test_cnf() {
    // println!("{}", conjunctive_normal_form("AB&!"));
    // // A!B!|
    // println!("{}", conjunctive_normal_form("AB|!"));
    // // A!B!&
    // println!("{}", conjunctive_normal_form("AB|C&"));
    // // AB|C&
    // println!("{}", conjunctive_normal_form("AB|C|D|"));
    // // ABCD|||
    // println!("{}", conjunctive_normal_form("AB&C&D&"));
    // // ABCD&&&
    // println!("{}", conjunctive_normal_form("AB&!C!|"));
    // // A!B!C!||
    // println!("{}", conjunctive_normal_form("AB|!C!&"));
    // // A!B!C!&&
    // println!("{}", conjunctive_normal_form("AB|CD|&"));
    // println!("{}", conjunctive_normal_form("AB&CD&|"));
    // println!("{}", conjunctive_normal_form("AB&CD&|"));
    // println!("{}", negation_normal_form("A!!!!"));
}

fn main() {
    test_cnf();
}
