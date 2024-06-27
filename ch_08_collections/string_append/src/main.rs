fn main() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2); // push_str does not take ownership of s2 so can still print s2
    println!("s2 is {s2}");
}
