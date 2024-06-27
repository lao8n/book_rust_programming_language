fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // overwrites value

    println!("{:?}", scores);

    scores.entry(String::from("Yellow")).or_insert(50); // only inserts if key doesn't exist
    scores.entry(String::from("Blue")).or_insert(50); // doesn't insert because key exists

    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // update based on old value
        *count += 1; // count is a mutable ref &mut V
    }

    println!("{:?}", map);
}

// rust uses the SipHash algorithm to provide resistance to Denial of Service attacks involves hash tables, trade-off of security to speed.
