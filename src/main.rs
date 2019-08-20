use crate::closure::Cacher;
use crate::trait_interface::Circle;

mod basic_syntax;
mod highorder_func;
mod trait_interface;
mod io_example;
mod container;
mod generics;
mod closure;
mod smartpointer;

fn main() {
//    io_example();
//    basic_types();
//    container();
//    generic();
    closure();
}


#[allow(dead_code)]
fn package_example() {
    highorder_func::entry();
}


#[allow(dead_code)]
fn trait_interf() {
    // struct
    let cir = Circle::new(1.0, 2.0, 5.0);
    // the following initialization method only can used for tuple struct,
    // tuple struct doesn`t has named field(attribute)
//    let cir1 = Circle(1.0, 2.0, 2.0);
    trait_interface::print_area(cir);
    println!();
    trait_interface::use_foobar();
    println!();
    trait_interface::use_childfoo();
}


#[allow(dead_code)]
fn io_example() {
//    file_in();
    io_example::file_out();
//    stdio_in();
//    stdio_out();
}


#[allow(dead_code)]
fn basic_types() {
//    simple();
//    slice();
    let word = basic_syntax::first_word("hellorust");
    println!("word is: {}", word)
}


#[allow(dead_code)]
fn container() {
    container::vector()
}


#[allow(dead_code)]
fn generic() {
    let list = vec![1, 2, 3, 4, 5];
    let largest = generics::largest_copy(&list);
    println!("the largest in the vector is {}", largest);
}


#[allow(dead_code)]
fn closure() {
    let mut calc = Cacher::new(
        |num| {
            println!("num is {}", num);
            num
        }
    );

    println!("calc is  {}", calc.value("hello"));
}
