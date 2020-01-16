mod basic_syntax;
mod concurrent;
mod container;
mod functional;
mod generics;
mod hybrid_programming;
mod io_example;
mod macro_learn;
mod oop;

fn main() {
    // io_example();
    // basic_types();
    // container();
    // generic();
    // functional();
    // concurrent();
    // oop();
    // hybrid_programming();
    // macro_learn();
    test_ptr();
}

#[allow(dead_code)]
fn test_ptr() {
    let x = vec![0, 1, 2, 3];
    let ptr = x.as_ptr();

    unsafe {
        println!("{}", ptr.read());
        //        assert_eq!(ptr.read(), [0,1,2,3]);
    }
}

fn trait_interf() {
    oop::trait_interface();
    oop::screen_run();
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
fn functional() {
    //    functional::entry();

    functional::closure();
}

#[allow(dead_code)]
fn concurrent() {
    use concurrent::ConcurrentExp;

    //    ConcurrentExp::thread_example();
    //    ConcurrentExp::channel_example();
    ConcurrentExp::mutex_exp()
}

#[allow(dead_code)]
fn oop() {
    //    oop::screen_run();

    oop::trait_interface();
}

#[allow(dead_code)]
fn hybrid_programming() {
    hybrid_programming::calling_c();
}

#[allow(dead_code)]
fn macro_learn() {
    macro_learn::customize_macro();
}
