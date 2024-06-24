use std::sync::mpsc; // multiple producer single consumer
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel(); // transmitter & receiver destructured

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap(); // errors if receiver has dropped - real application should handle not panic with unwrap
        // println!("val is {val}"); would error 
    });

    let received = rx.recv().unwrap(); // recv receives error when transmitter closes
    // try_recv doesn't block where it errors if no messages at that time - so could keep trying and then do something else
    println!("Got: {received}");
}