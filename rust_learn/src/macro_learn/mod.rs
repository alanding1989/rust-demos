use hello_macro_derive::HelloMacro;

pub trait HelloMacro {
    fn hello_macro();
}

// like decorator, but it is inline extract in compile time
// derive macro can only used for Struct and Enum.
#[derive(HelloMacro)]
struct Pancakes;

pub fn customize_macro() {
    Pancakes::hello_macro();
}
