fn main() {
    let v = vec![1, 2, 3];

    v[99];
}
// $ cargo run
//    Compiling panic v0.1.0 (file:///projects/panic)
//     Finished dev [unoptimized + debuginfo] target(s) in 0.27s
//      Running `target/debug/panic`
// thread 'main' panicked at src/main.rs:4:6:
// index out of bounds: the len is 3 but the index is 99
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace