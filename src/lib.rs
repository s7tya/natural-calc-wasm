use wasm_bindgen::prelude::*;
pub mod parser;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn run(source: String) -> String {
    source
        .split("\n")
        .map(|line| {
            if line.trim() == "" {
                return "".to_string();
            }

            parser::parser::calc(line)
                .map(|n| {
                    if n.fract() == 0.0 {
                        (n as i32).to_string()
                    } else {
                        n.to_string()
                    }
                })
                .unwrap()
        })
        .collect::<Vec<_>>()
        .join("\n")
}
