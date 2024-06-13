Performance
* No performance cost as monomorphization means that compiled code has concrete types

Restrictions
* Cannot implement external traits on external types, known as the orphan rule or coherence

Lifetimes
* another type of generic that ensure that references are valid as long as we need them to be
* most of the time lifetimes are implicit and inferred
* goal is to prevent dangling references

Borrow checker
* Used to compare scopes and determine whether all borrows are valid
```
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```