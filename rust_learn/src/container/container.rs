#[allow(unused_variables)]
pub fn vector() {
    let mut v1 = vec![1, 2, 3]; // macro_learn
    let v2 = vec![0; 10];

    for i in &mut v1 {
        *i = *i + 1;
        println!("i = {:?}", *i);
    }

    for i in &v2 {
        println!("i = {}", i);
    }
    println!("");
}
