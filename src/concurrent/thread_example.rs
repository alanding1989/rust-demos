use std::thread;
use std::time::Duration;

//<editor-fold desc="@ Tread low level concurrent">
pub fn thread_join_before_main<F>(main_func: F, s: &str)
    where F: Fn(&str) {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread! ", i);
            thread::sleep(Duration::from_millis(1));
        }
    }).join().unwrap();

    println!();
    main_func(s)
}

pub fn thread_join_after_main<F>(main_func: F, s: &str)
    where F: Fn(&str) {
    let handle = thread::spawn(||
        for i in 1..10 {
            println!("hi number {} from the spawned thread! ", i);
            thread::sleep(Duration::from_millis(1));
        }
    );

    main_func(&s);
    println!();

    handle.join().unwrap();
}
//</editor-fold>


