struct MyBox<T>(T); // tuple struct - struct has name but fields do not

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> { // DerefMut trait for mutable references
    type Target = T;

    fn deref(&self) -> &Self::Target { // doesn't deref instead it returns a plain reference which can then be dereferenced
        &self.0 // returns reference not the value because do not want to transfer ownership
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

// rust will use deref coercion if Deref trait is implemented
fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // equivalent to *(y.deref())

    let m = MyBox::new(String::from("Rust"));
    hello(&m); // otherwise would need to do hello(&(*m)[..]);
}