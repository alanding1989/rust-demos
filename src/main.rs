mod basic_syntax;
mod functional;
mod trait_interface;
mod io_example;
mod container;
mod generics;
mod concurrent;


fn main() {
//    io_example();
//    basic_types();
//    container();
//    generic();
//    closure();
    concurrent();
}


#[allow(dead_code)]
fn package_example() {
    functional::entry()
}


#[allow(dead_code)]
fn trait_interf() {
    use crate::trait_interface::Circle;
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
    use functional::Cacher;
    let mut calc = Cacher::new(
        |num| {
            println!("num is {}", num);
            num
        }
    );

    println!("calc is  {}", calc.value("hello"));
}


#[allow(dead_code)]
fn concurrent() {
    use std::thread;
    use std::time::Duration;

    let s = String::from("alan is the best !");

    let main_fc = |s: &str| for i in 1..5 {
        println!("hi, number {} from the main thread !", i);
        println!("{}", s);
        thread::sleep(Duration::from_millis(1));
    };

//    concurrent::thread_join_before_main(main_fc, &s);
    concurrent::thread_join_before_main(main_fc, &s);
    println!("s is {}", s);
}
