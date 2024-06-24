use std::sync::{Arc, Mutex};
use std::thread;

// Send marker trait = indicates that ownership of values can be transferred between threads, most rust types are Send
// but exceptions include Rc<T> and raw pointers
// Sync marker trait = indicates that it is safe to be referenced from multiple threads. Any type T is Sync if &T (immutable reference to T) is Send
// Marker traits don't have any methods to implement they are just a useful way to track whether types (and their compositions) enforce variants
fn main() {
    // can't use raw mutex as move counter once
    // can use reference counter as not concurrency safe
    let counter = Arc::new(Mutex::new(0)); // automatically referenced type
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter); // in practice counter is a primitive type so could use std::sync::atomic instead
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1; // this pattern is same as interior mutability with RefCell and Rc
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}