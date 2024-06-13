fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }
}

// want references because don't want to take ownership
// fn longest(x: &str, y: &str) -> &str { // doesn't compile without lifetime
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
// constraint - returned reference is valid as long as both parameters are valid
// only about helping borrow checker not actually chaning the lifetime of any values passed in or returned
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { // generic lifetime 'a gets a concrete lifetime which is smaller of x & y
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// // below would not compile
// fn main() {
//     let string1 = String::from("long string is long");
//     let result;
//     {
//         let string2 = String::from("xyz");
//         result = longest(string1.as_str(), string2.as_str());
//     }
//     println!("The longest string is {result}");
// }