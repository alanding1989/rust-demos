use hello_macro_derive::*;

trait HelloMacro {
    fn hello_macro();
}

// like decorator, but it is inline extract in compile time
// derive macro can only used for Struct and Enum.
#[derive(HelloMacro)]
struct Pancake;

pub fn customize_macro1() {
    Pancake::hello_macro();
}

// 为结构体自动实现方法
#[derive(Macro)]
struct PMacro;

pub fn customize_macro() {
    PMacro::proc_macro();
}
