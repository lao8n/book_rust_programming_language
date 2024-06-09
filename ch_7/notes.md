Crate = smallest amount of code rust compiler considers at a time, although crates can contain modules
Two types
1. Binary crate = program you can compile to an executable that yuo can run such as a command-line program or a server
2. Library crate = don't have a main function, don't compile to an executable - usually rustaceans mean library crate when they say crate.

Package = bundle of one or more crates. It contains a Cargo.toml file, it can have many binary crates but only one library crate.

Crate root = source file that compiler starts from - as default it is at `src/main.rs` or `src/lib.rs` or if multiple binary in `src/bin` directory

Modules cheat sheet = https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html

Put as much code in lib as possible and make binary just a user of the lib