fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    // add is defined as fn add(self, s: &str) -> String - takes ownership of self because self does have an &
    // rust deref coerces &String to &str i.e. &s2 to &s2[..]

    // if concatenating multiple strings use format! macro instead
    // remember
    // String = owned data, dynamic heap allocation
    // &str = borrowed data slice, immutable stack allocation, 
    let s = format!("{s1}, {s2}!");
}
