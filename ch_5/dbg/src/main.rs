#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // dbg macro takes ownership of the value and returns it, whereas println! takes a reference
        height: 50,
    };

    dbg!(&rect1);
}