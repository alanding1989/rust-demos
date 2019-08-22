pub fn simple() {
    let (a, mut b): (bool, bool) = (true, false);
    println!("a= {:?}, b = {:?}", a, b);
    b = true;
    assert_eq!(a, b);

    let x = 5;
    let raw = &x as *const i32;
    let points_at = unsafe { *raw };
    println!("points_at = {:?}", points_at);
}

// array slice
pub fn slice() {
    // NOTE: pay attention to the type
    let a = &[1, 2, 3] as *const [i32; 3];
    let b = &[1, 2, 3, 4];

    let c: [i32; 3] = [1, 2, 3];
    // this is slice
    let slice_c: &[i32] = &c[..];

    let show = |arr: &[i32]| {
        println!("this is {:?}", arr);
        for i in arr {
            println!("every i = {}", i);
        }
        println!();
    };
    show((unsafe { *a }).as_ref());
    show(&b[..]);
    show(&c);
    show(slice_c)
}

// &str String and char
pub fn first_word(s: &str) -> &str {
    // &str is literal string, stored as binary, always utf-8 encoding.
    // contains a ptr pointed to the byte start in the binary file, and
    // a length of the char. So &str has fixed length.
    // String is a struct allocated in the heap
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    return &s[..];
}
