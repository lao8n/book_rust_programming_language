fn main() {
    let v: Vec<i32> = Vec::new(); // type annotation required
    let v = vec![1, 2, 3]; // type annotation not required
    let mut v = Vec::new(); // type annotation not required
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    let third: &i32 = &v[2]; // reference to third element, note that this will panic if the index is out of bounds
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
}
