#![feature(proc_macro, wasm_custom_section)]

#[macro_use] extern crate stdweb;
extern crate regex;
extern crate wasm_bindgen;

mod router;

use router::Router;

fn main() {
    stdweb::initialize();
    let mut router: Router<String> = Router::new();
    router.add("products".to_string());
    router.add("to_delete".to_string());
    router.remove("to_delete".to_string());
    println!("{:?}", router.get_fragment());
    println!("{:?}", router);
    stdweb::initialize();
}
