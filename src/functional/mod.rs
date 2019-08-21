use closure::Cacher;
pub use highorder_func::entry;

mod closure;
mod highorder_func;


#[allow(dead_code)]
pub fn closure() {
    let mut calc = Cacher::new(
        |num| {
            println!("num is {}", num);
            num
        }
    );

    println!("calc is  {}", calc.value("hello"));
}
