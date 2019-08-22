#[allow(dead_code)]
pub fn entry() {
    let transform = add_one;
    let f0 = add_one(2132) * 2;
    let f1 = apply(add_one, 2);
    let f2 = apply(transform, 2);
    println!("{}, {}, {}", f0, f1, f2);

    // number-type suffix
    let box_fn = factory(1i32);
    // rust can auto dereference for us
    let b0 = box_fn(2i32) * 2;
    let b1 = (*box_fn)(2i32) * 2;
    let b2 = (&box_fn)(2i32) * 2;
    let b2 = (&&&*box_fn)(2i32) * 2;
    println!("{}, {}, {}", b0, b1, b2);
}


fn add_one(x: i32) -> i32 { x + 1 }

// func pointer
fn apply_f(f: fn(i32) -> i32, y: i32) -> i32 {
    f(y) * y
}

// func trait
fn apply<F>(f: F, y: i32) -> i32
    where F: Fn(i32) -> i32 {
    f(y) * y
}

fn factory(x: i32) -> Box<dyn Fn(i32) -> i32>
{
    Box::new(move |y| x + y)
}
