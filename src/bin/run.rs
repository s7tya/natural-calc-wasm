use std::env;

use natural_calc_wasm::{eval, parser::parser};

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!();
    }

    let prog = parser::program(&args[1]).unwrap();
    // println!("{:?}", prog);

    let mut evaluator = eval::Eval::new();
    let result = evaluator.eval(prog);
    println!("{:?}", result);
}
