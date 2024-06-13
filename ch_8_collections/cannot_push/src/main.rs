fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    v.push(6); // cannot borrow `v` as mutable because it is also borrowed as immutable
    // and pushing might involve re-allocating memory and copying the old elements to the new space

    println!("The first element is: {first}"); // doesn't compile
}
