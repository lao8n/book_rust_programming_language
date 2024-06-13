fn main() {
   let s = "hello"; // immutable
   let mut s = String::from("hello"); // mutable
   s.push_str(", world!");
   println!("{}", s);

   let s1 = String::from("hello");
   let s2 = s1; // this is a move o/w have double free error, s1 invalid
}// when s goes out of scope Rust calls drop automatically - called RAII in C++
