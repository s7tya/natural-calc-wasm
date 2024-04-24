use object::Object;
use wasm_bindgen::prelude::*;

pub mod ast;
pub mod eval;
pub mod object;
pub mod parser;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn run(source: String) -> Box<[JsValue]> {
    let ast = parser::parser::program(&source).unwrap();
    let mut eval = eval::Eval::new();
    let result = eval
        .eval(ast)
        .iter()
        .map(|item| {
            JsValue::from_str(&match item {
                Object::Void => "".to_string(),
                Object::Int(v) => v.to_string(),
                Object::Float(v) => v.to_string(),
            })
        })
        .collect::<Vec<_>>();

    result.into_boxed_slice()
}
