extern crate proc_macro;
use proc_macro::*;

#[proc_macro_attribute]
pub fn return_as_is(_attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{}\"", _attr.to_string());
    println!("item: \"{}\"", item.to_string());
    "fn answer() -> u32 { 42 }".parse().unwrap()
}

#[proc_macro_attribute]
pub fn route(_attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{}\"", _attr.to_string());
    println!("item: \"{}\"", item.to_string());
    "fn answer() -> u32 { 42 }".parse().unwrap()
}
