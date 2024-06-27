Two types of errors
1. Recoverable = `Result<T, E>` e.g. for file not found error
2. Unrecoverable = `panic!` macro e.g. for accessing a location beyond the end of an array

In Cargo.toml file if you want to abort rather than cleaning up the stack 
```[profile.release]
panic = 'abort'```