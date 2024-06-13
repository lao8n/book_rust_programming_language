fn main() {
    let s = String::from("hello");
    let s1 = s.clone(); // deep copy
    println!("s = {}, s1 = {}", s, s1);
}
