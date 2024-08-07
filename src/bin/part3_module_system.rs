use learn_rust::greet; // bring greet into scope

fn main() {
    // Module System
    // hello
    // ├── Cargo.toml
    // └── src
    //     ├── main.rs // the hello binary
    //     └── lib.rs // the hello library

    //learn_rust::greet(); // won't work because greet is not public, All members are private by default in library.
    // To fix this, we need to make greet public. We can do this by adding pub keyword in front of greet.
    // Now, we can call greet from the binary crate.
    learn_rust::greet();

    // Specifying the absolute path of something at every call site can be tedious.
    // We can use the use keyword to bring a path into scope. Similar to import in Scala/Java.

    greet(); // greet is now in scope

    // Standard library is imported by default.
    // use std::collections::HashMap;

    // Packages
    // * A Cargo package is a Cargo feature that lets you build, test, and share code.
    // * A package contains a Cargo.toml file that describes how to build the package.
    // * A package must contain at least one library or binary.
    // * A package can contain multiple libraries and binaries.
    // Once you have identified a package, you can use the package name to import the package.
    // Go to cargo.toml and add the package name under dependencies.
    // For example, if you want to use the rand package, add rand = "0.8.5" under dependencies.
    // [dependencies]
    // rand = "0.8.3"

    let random_number: i32 = rand::random();
    println!("Random number: {}", random_number)
}
