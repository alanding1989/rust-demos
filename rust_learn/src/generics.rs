//<editor-fold desc="@ Generic functions">
#[allow(unused_variables)]
pub fn largest<T: PartialOrd>(list: &[T]) -> &T
{
    let mut _largest = &list[0];
    for item in list.iter() {
        if item > _largest {
            let _largest = item;
        }
    }
    _largest
}


// only can used for primitive types stored in stack
#[allow(unused_variables)]
pub fn largest_copy<T: PartialOrd + Copy>(list: &[T]) -> T
{
    let largest = list[0];
    for &item in list.iter() {
        if item > largest {
            let largest = item;
        }
    }
    largest
}


// can used for all types stored in stack or allocated in heap
// but will cause runtime cost and heap memory consumption
#[allow(unused_variables)]
pub fn largest_clone<T: PartialOrd + Clone>(list: &[T]) -> T
{
    let _largest = list[0].clone();
    for item in list.iter() {
        if item > &_largest {
            let _largest = item.clone();
        }
    }
    _largest
}
//</editor-fold>


