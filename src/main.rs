// mod arithmetic;
mod boole;
// mod gray_code;
// mod set;
// mod space;

// use arithmetic::adder::adder;
// use arithmetic::multiplier::multiplier;
// use boole::boolean_evaluation::eval_formula;
// use boole::truth_table::print_truth_table;
// use set::powerset::powerset;
// use space::curve::map;
// use gray_code::gray_code;
use boole::negation_normal_form::negation_normal_form;

fn test_negation_normal_form() {
    println!("=== Exercise 05 - Negation Normal Form ===");
    println!(
        "negation_normal_form(\"AB&!\")   = {}",
        negation_normal_form("AB&!")
    );
    println!(
        "negation_normal_form(\"AB|!\")   = {}",
        negation_normal_form("AB|!")
    );
    println!(
        "negation_normal_form(\"AB>\")    = {}",
        negation_normal_form("AB>")
    );
    println!(
        "negation_normal_form(\"AB=\")    = {}",
        negation_normal_form("AB=")
    );
    println!(
        "negation_normal_form(\"AB|C&!\") = {}",
        negation_normal_form("AB|C&!")
    );
}

fn main() {
    test_negation_normal_form();
}
