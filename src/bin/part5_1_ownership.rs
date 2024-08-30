fn main() {
    // Ownership

    // Backstory
    // Ownership is the reason why I want to learn Rust as well as made me avoid it for so long.
    // Coming from Scala language, I am used to the garbage collector and the JVM taking care of memory management.
    // I don't have worry about explicitly doing memory management.
    // Last time I did memory management was in C and C++ and that too in my academic days which was in 2010.
    // To be honest, after learning Scala and Java, I never wanted to go back to C and C++.

    // Ownership Introduction
    // Ownership is what makes safety guarantees possible. Make Rust so different from other languages.
    // Ownership is what makes all those informative compiler error messages possible and necessary.
    // There are 3 rules:
    // 1. Each value in Rust has a variable that's called its owner.
    // 2. There can only be one owner at a time. (no variable can share ownership)
    // 3. When the owner goes out of scope, the value will be dropped.

    // Ownership in action
    // let s1 = String::from("abc");
    // let s2 = s1;
    // println!("{}", s1); // This will not compile. Rust will throw an error.
    // This is because Rust does not allow two variables to point to the same memory location.
    // Error message: value borrowed here after move
    // This is called a move. s1 has moved to s2. s1 is no longer valid. This is not a shallow copy.

    // What's happening behind the scenes?
    // We've got stack and heap. Stack is fast and heap is slow.
    // Stack is In-Order, Fixed Size, LIFO, and Fast.
    // Heap is Out-of-Order, Variable Size, unordered, and Slow.

    // Here we create s1.
    //           Stack              Heap
    //        |-----|---|        |-----|
    //        | ptr |   | -----> | a   |
    // s1 --> | len | 3 |        | b   |
    //        | cap | 3 |        | c   |
    //        |-----|---|        |-----|
    // A pointer, a length, and a capacity get pushed onto the stack.
    // The pointer points to the memory on the heap where the value is stored.
    // This all together is the value of the string s1.
    // Then we create s2 and move the value of s1 to s2.
    //           Stack              Heap
    //        |-----|---|        |-----|
    //        | ptr |   | -----> | a   |
    // s2 --> | len | 3 |        | b   |
    //        | cap | 3 |        | c   |
    //        |-----|---|        |-----|
    // Here the pointer, length, and capacity get copied to s2 and pushed as a new value onto the stack.
    // If we stopped here then we would have two pointers pointing to the same memory location on the heap.
    // Then memory safety would be compromised.
    // Rust prevents this by invalidating s1. s1 is no longer valid.
    // s1 still exists on the stack but the compiler just consider s1 now uninitialized and won't let us use it.
    // This is called a move.

    // What if we don't want to move the value but copy it instead?
    // let s1 = String::from("abc");
    // let s2 = s1.clone();
    // println!("s1 = {}, s2 = {}", s1, s2);
    // This will work. Rust will copy the value of s1 to s2.

    // Why it is called clone and not copy?
    //           Stack              Heap
    //        |-----|---|        |-----|
    //        | ptr |   | -----> | a   |
    // s1 --> | len | 3 |        | b   |
    //        | cap | 3 |        | c   |
    //        |-----|---|        |-----|
    //           Stack              Heap
    //        |-----|---|        |-----|
    //        | ptr |   | -----> | a   |
    // s2 --> | len | 3 |        | b   |
    //        | cap | 3 |        | c   |
    //        |-----|---|        |-----|
    // Here the pointer, length, and capacity get copied to s2 and pushed as a new value onto the stack.
    // Rust reserve the term copy for when the stack data is copied.
    // If there is heap data and pointer updates involved then it is called clone.
    // In other languages, clone is called deep copy.

    // What happen when a value is dropped?
    // When value is dropped that means the destructor is called if there is one.
    // The heap value is immediately freed.
    // The stack value is popped off the stack.
    // So no leaks, no dangling pointers which makes programmers happy.

    // Another move situation
    // let s1 = String::from("abc");
    // do_something(s1);
    // println!("{}", s1); // This will not compile. Rust will throw an error.
    // fn do_something(s: String) {
    //     println!("{}", s);
    // }
    // This is because s1 has moved to s in the function do_something.
    // Which means s1 is no longer valid in the main function.

    // So what do we do?
    // One option is to move the value back to s1.
    // let mut s1 = String::from("abc");
    // do_something(s1);
    // println!("{}", s1);
    // fn do_something(s: String) -> String {
    //     println!("{}", s);
    //     s
    // }
    // This will work but it is not idiomatic Rust.

    // The idiomatic way is to pass a reference.
}
