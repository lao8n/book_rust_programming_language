// 3 ways a closure can finish
// 1. FnOnce = all closures that can be called once, all closures implement at least this trait as all can be called. A closure that moves captured values out of its body will only implement FnOnce
// 2. FnMut = closures that don't move captured values of their body, but that might mutate captured values, they can be called more than once
// 3. Fn = closures taht don't move captured values and don't mutate

enum Option<T> {
    None,
    Some(T),
}

impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}