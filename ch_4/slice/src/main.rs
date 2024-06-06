// slice is a reference to a contiguous sequence of elements in a collection and does not have ownership
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // error! needs a mutable reference to truncate the string, but already have immutable reference word

    println!("the first word is: {}", word); 
}

fn first_word(s: &String) -> &str { // string slice - for argument &str is better than &String because both can be passed in
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { // byte literal syntax
            return &s[0..i]; // &s[..i] is equivalent
        }
    }
    &s[..] // &s[0..s.len()] is equivalent
}
// string literals are slices