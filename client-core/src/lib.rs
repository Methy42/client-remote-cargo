extern crate proc_macro;
use proc_macro::{TokenStream};

#[proc_macro_derive(Metatron)]
pub fn metatron(input: TokenStream) -> TokenStream {
    let result = "fn answer() -> u32 { 42 }".parse().unwrap();
    println!("metatron {}", input.to_string());
    return result;
}

#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {
    println!("make_answer {}", _item.to_string());
    "fn answer() -> u32 { 42 }".parse().unwrap()
}
