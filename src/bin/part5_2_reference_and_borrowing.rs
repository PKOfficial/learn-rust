fn main() {
    // Reference and Borrowing
    // Now we gone over ownership, let's talk about reference and borrowing.
    // If you are here first, then you should go back and read about ownership.
    // Checkout src/bin/part5_1_ownership.rs

    // Instead of moving the value, let's use a reference.
    // fn do_something(s: &String) {
    //     println!("{}", s);
    // }
    // If you notice, we are using & to create a reference.
    // We are also using &String instead of String.
    // let s1 = String::from("abc");
    // let s = do_something(&s1);
    // Here we pass a reference to s1 to the function do_something and s1 retains ownership.
    // do_something borrows a reference to the value. The reference, not the value, get moved to the function.
    // At the end of the function, the reference goes out of scope and reference gets dropped.
    // So borrowing ends when the reference goes out of scope.

    //      s            s1
    // |-----|---|    |-----|---|
    // | ptr |   | -> | ptr |   |
    // |-----|---|    | len | 3 |
    //                | cap | 3 |
    //                |-----|---|

    // Under the hood
    // When we create a reference to s1, Rust creates a pointer to s1.
    // We mostly don't need to worry about the pointer.
    // Rust automatically handles their creation and destruction for the most part.
    // It also make sure they're always valid using concepts like lifetimes.

    // Lifetime can be summed up as a rule that references must always be valid.
    // That means compiler won't let you create a reference to outlives the data it points to and you can never point to null.

    // Reference default to immutable even if the value is being referenced is mutable.
    // If we want to make the reference mutable then we can use &mut.
    // fn do_something(s: &mut String) {
    //     s.push_str("def");
    // }
    // let mut s1 = String::from("abc");
    // do_something(&mut s1);
    // println!("{}", s1);
    // Here we are passing a mutable reference to s1 to the function do_something.
    // We are also using &mut String instead of &String.

    // Dot Operator for a method or a field auto-dereferences down to the actual value.
    // So when you're dealing with the dot operator you don't have to worry about something is value
    // or reference or reference of reference.
    // If we manually dereference then we have to use *.
    // fn do_something(s: &mut String) {
    //     (*s).push_str("def");
    // }
    // Dereference operator has lower precedence than dot operator. So sometimes we need to use parentheses.
    // With most other operators, like assignment operator for example you'll need to manually dereference.
    // fn do_something(s: &mut String) {
    //     *s = String::from("def");
    // }
    // Here I'm dereferencing s and replacing it with a new value.

    // x: &i32 is a reference
    // *x an immutable i32
    // &mut x is a mutable reference
    // *x = 5 a mutable i32

    // Rust has a special rule to keep us safe.
    // You can have either one mutable reference or multiple immutable references but not both.

    // Why this is helpful especially in multithreaded programming?
    // Thread A
    //   ref1
    //     |----|--|
    //     | ptr|  |
    //     |----|--|  ---------->    Thread C
    //                                  s1
    // Thread B       -------->  |----|----|      |-----|
    //   ref2                    | ptr |   | --> | a   |
    //     |----|--|             | len | 3 |     | b   |
    //     | ptr|  |             | cap | 3 |     | c   |
    //     |----|--|             |----|----|     |-----|

    // If references to a variable may exist in multiple threads it starts to make it pretty
    // obvious why it's not safe to have multiple mutable references to the same variable at the
    // same time without some type of synchronization or locking mechanism.
}
