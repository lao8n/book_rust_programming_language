fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    // if can copy because has Copy trait, otherwise takes ownership
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0); // copied for Option<i32> rather than Option<&i32>
    for (key, value) in &scores { // unordered
        println!("{key}: {value}");
    }
}
