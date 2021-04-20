use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern{
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str){
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn add(x: i32, y:i32){
    let num: i32 = x + y;
    alert(&format!("La suma da {}",  num.to_string()));
}