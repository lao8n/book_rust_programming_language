// 3 ways a closure can finish
// 1. FnOnce = all closures that can be called once, all closures implement at least this trait as all can be called. A closure that moves captured values out of its body will only implement FnOnce
// 2. FnMut = closures that don't move captured values of their body, but that might mutate captured values, they can be called more than once
// 3. Fn = closures taht don't move captured values and don't mutate

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    list.sort_by_key(|r| r.width);
    println!("{list:#?}");
}