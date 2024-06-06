fn main() {
    let mut s = String::from("hello");

    change(&mut s);
    problem();
}
// mutable references have one big restriction: you can have only one mutable reference to a particular piece of data in a particular scope
// cannot have other mutable or immutable references
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn problem(){
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
}