use mpi;
use thread_example;

mod thread_example;
mod mpi;


pub fn thread_example() {
    use std::thread;
    use std::time::Duration;

    let s = String::from("alan is the best !");

    let main_fc = |s: &str| for i in 1..5 {
        println!("hi, number {} from the main thread !", i);
        println!("{}", s);
        thread::sleep(Duration::from_millis(1));
    };

    thread_example::thread_join_after_main(main_fc, &s);
    thread_example::thread_join_before_main(main_fc, &s);
    println!("s is {}", s);
}


pub fn channel_example() {
    mpi::channel();
}
