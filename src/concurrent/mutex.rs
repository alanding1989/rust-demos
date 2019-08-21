use std::sync::{Arc, Mutex};
use std::thread;

pub fn mutex_10() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num = *num + 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // why need to transfer to iter first, then can map ???
//    handles.into_iter().map(|handle| {
//        handle.join().unwrap()
//    });

    println!("Result: {}", *counter.lock().unwrap());
}