#![deny(unused_must_use)]

mod error;
mod parser;
mod span;
mod util;
mod vm;

use lasso::Rodeo;
use parser::{
    lexer::{Lexer, Token},
    Parser,
};
use vm::Vm;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub fn bluh(s: String) {
    let mut rodeo = Rodeo::new();
    let mut parser = Parser::new(&s, &mut rodeo);
    let ast = match parser.parse_block(true) {
        Ok(ast) => ast,
        Err(e) => {
            console_log!("{:#?}", e);
            return;
        }
    };

    let mut vm = Vm::new();
    match vm.run_block(&ast, true, &mut rodeo) {
        Ok(_) => {}
        Err(e) => {
            console_log!("{:#?}", e);
        }
    };
}
