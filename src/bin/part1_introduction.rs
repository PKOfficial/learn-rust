fn main() {
    // Introduction to Rust
    // Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.
    // Rust is a statically typed language, which means that it must know the types of all variables at compile time.
    // The compiler can usually infer what type we want to use based on the value and how we use it.
    // We can also explicitly define the type using a colon and then the type name.

    // Rust is Awesome
    // Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.

    // High-level programming languages like Python, Ruby, and JavaScript are great for writing code quickly and concisely
    // but they are slow and have a lot of overhead.

    // Low-level programming languages like C and C++ are great for writing code that runs quickly and with minimal overhead
    // but they are difficult to write and maintain. Also difficult to write safe code.

    // Origin of Rust
    // Rust was created by Graydon Hoare at Mozilla Research in 2010.
    // Mozilla started sponsoring the project in 2009.
    // Version 1.0 was released in 2015.

    // Why Mozilla Sponsored System Programming Language?
    // Because they were not happy with the performance of C++ in their browser (Firefox).
    // Firefix Quantum was written in Rust and it is 2x faster than the previous version.

    // Cargo
    // Cargo is a package manager and build system for Rust.
    // Cargo is a build system and package manager that makes it easy to compile, test, and run Rust projects.
    // Cargo is also a test runner. It runs your tests and reports the results.
    // Cargo is document generator. It generates documentation for your code.
    // cargo new hello
    // tree hello
    // hello
    // ├── Cargo.toml
    // └── src
    //     └── main.rs
    // 1 directory, 2 files

    // Cargo.toml
    // [package]
    // name = "hello"
    // version = "0.1.0" // Semantic Versioning - Major.Minor.Patch
    // edition = "2018"
    // [dependencies] // Dependencies are listed here

    // src/main.rs
    // fn main() {
    //     println!("Hello, world!");
    // }

    // cargo run
    // Hello, world!

    // cargo run --release

    // Variables
    // Variables are immutable by default.
    // let x = 5; // here x is variable and 5 is value. Type annotation is not required. Rust can infer the type.
    // let x: i32 = 5 // here x is variable and 5 is value. i32 is type annotation.
    // let (b, c) = (1, 2); // here b and c are variables and 1 and 2 are values. Tuple pattern.

    // Why immutable by default? Because it helps in writing safe code.
    // Safety, Concurrency, and Performance are the main goals of Rust.
    // There are lots of bugs that can happen because of mutable state.
    // Data that never changes can be shared across threads without locks.
    // Compiler can also do extra optimizations because it knows the data will never change.

    // let x = 5;
    // x = 6; // cannot assign twice to immutable variable `x`

    // let mut x = 5;
    // x = 6; // this is fine

    // const
    // const MAX_POINTS: u32 = 100_000;
    // * Snake case is used for constants.
    // * Type annotation is required for constants.
    // * Constants are always immutable.
    // * It must be a constant expression, not the result of a function call.
    // Why use const?
    // * We can place a constant outside a function at module scope and use it anywhere we want.
    // * Const value are inlined at compile time therefore they are really fast.

    // Scope
    // Scope of a variable is the range within the program where that variable is valid.
    // let x = 5; // x is valid from here
    // { // start a new scope
    //     let y = 10; // y is valid from here
    //     println!("{}", y); // y is valid here
    // } // y is no longer valid
    // println!("{}", x); // x is valid here

    // Shadowing in Scope
    // Shadowing is when we declare a new variable with the same name as a previous variable.
    // let x = 5;
    // {
    //     let x = 10;
    //     println!("{}", x); // 10
    // }
    // println!("{}", x); // 5

    // let mut x = 5; // x is mutable
    // let x = x; // x is now immutable

    // Shadow a variable to different type
    // let x = "hello";
    // let x = x.len(); // x is now a number
    // This is useful when we want to change the type of a variable but keep the same name.
    // Use cases like: data conversion, data transformation, etc.

    // Memory Safety
    // Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.
    // Rust guarantees memory safety at compile time.
    // Variable must be initialized before use.
    // let x: i32; // expected `i32`, found `()`
    // println!("{}", x); // use of possibly uninitialized variable: `x`

    // let x: i32;
    // if true {
    //     x = 5;
    // }
    // println!("{}", x); // use of possibly uninitialized variable: `x`
    // Conditional evaluation is handled at runtime, not at compile time.
    // Compiler can't guarantee that x will be initialized.

    //let x: i32;
    //if true {
    //    x = 5;
    //} else {
    //    x = 6;
    //}
    //println!("{}", x); // This will work because x is initialized in both branches.
    // x is guaranteed to be initialized before it is used.
    // As long as compiler guarantees that x is initialized before it is used, it is memory safe.

    // Functions
}