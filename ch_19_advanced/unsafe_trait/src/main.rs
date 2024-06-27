// e.g. if want composability of Send and Sync types but are using raw pointer.
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

fn main() {}