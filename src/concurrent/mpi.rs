use std::sync::mpsc;
use std::thread;

pub fn channel() {
    let (sx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        sx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got : {}", received);
}
