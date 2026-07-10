//mod arithmetic;
//mod boole;
//mod set;
mod space;
//mod gray_code;

//use arithmetic::adder::adder;
//use arithmetic::multiplier::multiplier;
//use boole::boolean_evaluation::eval_formula;
//use boole::truth_table::print_truth_table;
//use set::powerset::powerset;
use space::curve::map;
//use gray_code::gray_code;

fn main() {
    println!("{}", map(65535, 65535));
}
