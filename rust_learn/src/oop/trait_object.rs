pub trait Draw {
    fn draw(&self);
}

//<editor-fold desc="@ Screen">
// generic T just can stand for those 'same' type implemented Draw
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T: Draw> Screen<T> {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// Box<dyn Draw> can accept 'different' types which implemented Draw.
// it`s like duck type.
pub struct Screen1 {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen1 {
    pub fn run(&self) {
        for component in self.components.iter() {
            // pay attention to the type of component, *component, **component,
            // Rust auto dereference for us.
            // (**component).draw()
            component.draw()
        }
    }
}
//</editor-fold>

//<editor-fold desc="@ Components">
pub struct Button {
    // doesn`t implement new() method, so fields need to be public.
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("This is Button! ");
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("This is SelectBox! ");
    }
}
//</editor-fold>
