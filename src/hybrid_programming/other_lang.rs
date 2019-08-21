extern "C" {
    fn abs(input: i32) -> i32;
}

pub fn c_fun() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
