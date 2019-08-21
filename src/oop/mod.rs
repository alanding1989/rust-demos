use crate::oop::trait_object::{Button, SelectBox};

mod trait_object;

pub fn screen_run() {
    use trait_object::Screen1;
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
