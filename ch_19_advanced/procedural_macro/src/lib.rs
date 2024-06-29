// 3 types of procedural macros that accept some code as an input and produce other code.
// 1. custom #[derive] macros that specify code added on structs and enums
// 2. attribute-like macros on any item
// 3. function-like macros that operate on tokens specified as their arguments

// need to define procedural macros in its own crate
// use proc_macro; 

// #[some_attribute]
// pub fn some_name(input: TokenStream) -> TokenStream {
// }

pub trait HelloMacro {
    fn hello_macro();
}