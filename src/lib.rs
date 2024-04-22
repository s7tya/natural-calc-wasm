use wasm_bindgen::prelude::*;
pub mod parser;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn run(source: String) -> String {
    format!("{:?}", parser::parser::calc(&source).unwrap())
}
