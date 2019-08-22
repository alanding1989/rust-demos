use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn channel_one_val() {
    let (sx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        sx.send(val).unwrap();
        // NOTE: the following will be error, once the value send,
        // it can`t be used again in current scope.
        // value will move to the receiver.
//        println!("val is {}", val);
    });

    let received = rx.recv().unwrap();
    println!("Got : {}", received);
}


pub fn channel_several_vals() {
    let (sx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            sx.send(val).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });


//    for received in rx.try_recv() {
//        println!("Got {}", received);
//    }

//    for received in rx.try_iter() {
//        println!("Got {}", received);
//        thread::sleep(Duration::from_millis(500));
//    }

    while true {
        if let Some(v) = rx.try_recv().ok() {
            println!("Got {}", v);
            for i in 1..5 {
                println!(" val is {}", i);
                thread::sleep(Duration::from_millis(500));
            }
        } else {
            println!("No msg yet !");
            thread::sleep(Duration::from_secs(2));
        }
    }
}
