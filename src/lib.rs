use wasm_bindgen::prelude::*;
pub mod parser;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn run(source: String) -> String {
    parser::parser::calc(&source)
        .map(|n| {
            if n.fract() == 0.0 {
                (n as i32).to_string()
            } else {
                n.to_string()
            }
        })
        .unwrap()
}
