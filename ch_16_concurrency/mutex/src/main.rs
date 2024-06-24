use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);
     {
        let mut num = m.lock().unwrap(); // Mutex<T> is a smart pointer call MutexGuard wrapped in a LockResult
        *num = 6; // lock is released automatically thanks to MutexGuard Drop implementation
     }

     println!("m = {m::?}");
}
