use trait_object::{Button, SelectBox};
use trait_object::Screen1;

mod trait_object;


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
