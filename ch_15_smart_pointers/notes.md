Pointer = variable that contains an address in memory
Reference = type of pointer that borrows the value they point to and indicated with a `&`
Smart pointers = pointers with additional metadata and capabilities. Implemented using structs with the `Deref` and `Drop` traits where the former allows a smart pointer to act like a reference and latter allows you to customise the code that runs when smart pointer goes out of scope
Reference counting smart pointer = like C++ shared pointer it allows data to have multiple owners and when no owners remain clean up the data.

Types
* `Box<T>` for allocating values on the heap, single ownership although immutable and mutable borrows at compile time
* `Rc<T>` reference counting type that enables multiple ownership
* `Ref<T>` and `RefMut<T>` accessed through `RefCell<T>` that enforces the borrowing rules at runtime instead of compile time.