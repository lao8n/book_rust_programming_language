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

Lifetime elision rules - cases that the compiler considers where do not need to write lifetimes explicitly
* input lifetimes = lifetimes on function and method parameters
* output lifetimes = lifetimes on return values

1. rule compiler assigns a lifetime to each parameter that's a reference e.g. `fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`
2. if there is one input lifetime parameter that lifetime is assigned to all output life parameters `fn foo<'a>(x: &'a i32) -> &'a i32`
3. if there are multiple input lifetime parameters but one of them is `&self` or `&mut self` therefore this is a method and the lifetime of `self` is assigned to all output lifetime parameters