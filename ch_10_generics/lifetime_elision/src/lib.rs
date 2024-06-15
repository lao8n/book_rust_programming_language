// do not need lifetime here .. originally would have had to write
// fn first_word<'a>(s: &'a str) -> &'a str {...}
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}