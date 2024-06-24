use trait_obj::Draw;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}

use trait_obj::{Button, Screen};
// uses nominal typing with explicit implementation of a trait
// unlike strutural typing in golang or duck typing in python
// however unlike generics where concrete types are replaced at compile time via monomorphization doing static dispatch
// dynamic dispatch is when compiler doesn't know what method you are calling at compile time, instead it uses pointer
// in trait object to know which method to call
fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}