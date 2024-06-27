#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // associated functions
    fn area(&self) -> u32 { // method because has self as parameter
        self.width * self.height // self is alias for self: &Self
    }

    fn square(size : u32) -> Self { // associated function but not a method
        Self { width: size, height: size}
    }
}

fn main(){
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area() // not &rect1.area() because area takes a reference to self
    );
}