use crate::garden::vegetables::Asparagus; // can now just type Asparagus

pub mod garden; // declare a module - if this was not in src/main.rs it would be a submodule
// module is private by default, pub makes it public
fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}