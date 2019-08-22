use std::f64::consts::PI;
use std::fmt::Debug;

// two trait`s method has same name foo(),
// pay attention to the grammar.
pub fn use_foobar() {
    <BazFooBar as FooBar>::foo(&BazFooBar);
    <BazFooBar as Foo>::foo(&BazFooBar);
}

pub fn use_childfoo() {
    BazChildFoo.foobar();
    println!();
    BazChildFoo.foo()
}


//<editor-fold desc="simple trait and impl">

//<editor-fold desc="@ Trait abstract interface">
pub trait HasArea {
    fn area(&self) -> f64;
}

pub struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        PI * (self.radius * self.radius)
    }
}

impl Circle {
    pub fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle { x: x, y: y, radius: radius }
    }
}


pub struct Square {
    x: f64,
    y: f64,
    side: f64,
}

impl HasArea for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

impl Square {
    pub fn new(x: f64, y: f64, side: f64) -> Square {
        Square { x: x, y: y, side: side }
    }
}
//</editor-fold>


//<editor-fold desc="@ Generic function, Trait constraint">
pub fn print_area(shape: &Box<dyn HasArea>) {
    println!("This shape has an area of {}", shape.area());
}

// multi trait constraint
#[allow(unused_must_use)]
fn foo<T: Clone, K: Clone + Debug>(x: T, y: K) {
    x.clone();
    y.clone();
    println!("y = {:?}", y);
}

#[allow(unused_must_use)]
fn bar<T, K>(x: T, y: K) where T: Clone, K: Clone + Debug {
    x.clone();
    y.clone();
    println!("y = {:?}", y);
}
//</editor-fold>

//</editor-fold>


// default method
trait Foo {
    fn foo(&self);
    // default method
    fn bar(&self) { println!("We called bar."); }
}

// same method foo() with Foo
trait FooBar {
    fn foo(&self);
    fn foobar(&self);
}


// inheritance
trait ChildFoo: Foo {
    fn foobar(&self);
}

struct BazChildFoo;

// ChildFoo has super Trait Foo, struct want to impl ChildFoo also need to
// impl Foo.
impl ChildFoo for BazChildFoo {
    fn foobar(&self) {
        println!("this is ChildFoo inherit from Foo in BazChildFoo");
    }
}

impl Foo for BazChildFoo {
    fn foo(&self) {
        println!("this is Foo impl for BazChildFoo");
    }
}


struct BazFooBar;

impl Foo for BazFooBar {
    fn foo(&self) { println!("foo"); }

    // default method in Foo
    // fn bar(&self) { println!("We called bar."); }
}

impl FooBar for BazFooBar {
    fn foo(&self) { println!("this is FooBar`s foo method"); }

    fn foobar(&self) { println!("foobar"); }
}

