// calling C ABI, must define func proto first
extern "C" {
    fn abs(input: i32) -> i32;
}

#[allow(dead_code)]
pub fn calling_c() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}


// called from C
#[no_mangle]
#[allow(dead_code)]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C lang");
}
