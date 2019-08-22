use trait_interface::{Circle, Square};
use trait_object::{Button, SelectBox};
use trait_object::Screen1;

use crate::oop::trait_interface::HasArea;

mod trait_object;
mod trait_interface;


pub fn screen_run() {
    let screen = Screen1 {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    "Yes".to_string(),
                    "Maybe".to_string(),
                    "No".to_string(),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: "ok".to_string(),
            })
        ]
    };

    screen.run();
}

pub fn trait_interface() {
    // Struct initialization
    let shapes: Vec<Box<dyn HasArea>> = vec![
        Box::new(Circle::new(1.0, 2.0, 5.0)),
        Box::new(Square::new(2.0, 3.0, 10.0)),
    ];

    for x in shapes.iter() {
        trait_interface::print_area(x);
    }

//    trait_interface::print_area(Circle::new(1.0, 2.0, 5.0));
//    trait_interface::print_area(Square::new(2.0, 3.0, 10.0));

    // the following initialization method only can used for tuple struct,
    // tuple struct. which doesn`t has named field(attribute).
    // # Example:  let cir1 = Circle(1.0, 2.0, 2.0);

//    println!();
//    trait_interface::use_foobar();
//    println!();
//    trait_interface::use_childfoo();
}
