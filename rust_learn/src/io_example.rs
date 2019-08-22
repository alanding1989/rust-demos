use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read, Write};
use std::path::Path;

#[allow(unused_must_use)]
pub fn stdio_in() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("WTF!");
    println!("You typed: {}", input.trim())
}

#[allow(unused_must_use)]
pub fn stdio_out() {
    // print, println can`t output the strings, hasn`t flush new line
    print!("请输入一个字符串: ");
    // must add the following
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取失败");
    print!("您输入的字符串是: {}\n", input)
}


#[allow(unused_must_use)]
pub fn file_in() {
    // 创建一个文件路径
    let path = Path::new("/home/alanding/0_Dev/sparktest.txt");
    let display = path.display();


    // 基本处理方法
    // 打开文件只读模式，返回一个`io::Result<File>`类型
    let mut file = match File::open(path) {
        // 处理打开文件可能潜在的错误
        Err(why) => panic!("couldn`t open {}: {}", display, why),
        Ok(file) => file
    };

    // 文件输入数据到字符串，并返回 `io::Result<usize>`型
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Ok(_) => print!("{} contains:\n{}", display, s),
        Err(why) => panic!("couldn`t read {}: {}", display, why)
    }
}


#[allow(unused_variables)]
pub fn file_in_high() {
//    let f = File::open(&path).expect("can`t open");
    let path = Path::new("/home/alanding/0_Dev/rustfileio.txt");

    let file = File::open(path).map_err(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(path).unwrap_or_else(|error| {
                panic!("tried to create a file, but there is a problem {:?}", error)
            })
        } else {
            panic!("there was a problem opening the file: {:?}", error)
        }
    });
}


#[allow(unused_must_use)]
pub fn file_out() {
    // 创建一个文件路径
    let path = Path::new("/home/alanding/0_Dev/rustfileio.txt");
    let display = path.display();

    let mut file = match File::create(path) {
        Err(why) => panic!("couldn`t creat {}: {}", display, why),
        Ok(file) => file
    };

    match file.write_all("alan is the best".as_bytes()) {
        Err(why) => panic!(" {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display)
    };
}
