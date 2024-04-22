use std::env;

use natural_calc_wasm::parser::parser;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!();
    }

    println!("{:?}", parser::calc(&args[1]));
}
