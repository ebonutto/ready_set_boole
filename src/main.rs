//mod arithmetic;
mod boole;
//mod gray_code;

//use arithmetic::adder::adder;
//use arithmetic::multiplier::multiplier;
//use boole::boolean_evaluation::eval_formula;
use boole::truth_table::print_truth_table;
//use gray_code::gray_code;

fn main() {
    println!("Hello, world!");
    print_truth_table("AB&C|");
}
