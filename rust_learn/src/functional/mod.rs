use std::ops::Add;

use closure::Cacher;

mod closure;
mod highorder_func;


#[allow(dead_code)]
pub fn closure() {
    let mut calc = Cacher::new(
        |kstr: &str| {
            println!("The search key is {}", kstr);
            kstr.to_string().add(" the suffix")
        }
    );

//  pass by value -- move
//  let mut nval = String::new();
//  println!("\nThe value is {}", calc.get("hello", nval));


//  pass by pointer, unique pointer
    let ptr = calc.get_boxptr("hello");
    let ptr2 = ptr.clone();

    println!("\nThe value1 is {:?},
              \nThe value2 is {:?},
              \nThe value3 is {:?}\n\n",
             ptr.as_ptr(),
             ptr2.as_ptr(),
             (*ptr).clone().as_ptr()
    );


// pass by pointer, shared pointer
    let ptr = calc.get_rcptr("hello");
    let ptr2 = ptr.clone();
    println!("\nThe value1 is {:?},
              \nThe value2 is {:?},
              \nThe value3 is {:?}",
             ptr.as_ptr(),
             ptr2.as_ptr(),
             (*ptr).clone().as_ptr())
}

