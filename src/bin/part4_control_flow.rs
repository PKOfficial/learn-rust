fn main() {
    // Control Flow
    // You don't need parentheses around conditions in Rust.
    // Rust does not like type coercion. You must always provide a boolean condition.
    // Chain Conditions with `else if` and `else`.
    // if is expression not statement. It returns a value.

    // Four things to note:
    // 1. There are no semicolons after the branch values to make it so that the value gets returned from the blocks as tail expressions.
    // 2. We can't use return for this purpose, even if we wanted to, because return is a statement and we need an expression.
    // 3. All the blocks must return the same type. Rust is strongly typed and will not compile if the types are different.
    // 4. There is semi-colon after the if condition. If we put a semi-colon after the if condition, it will become a statement and Rust will not compile.

    let num = 6;
    let msg = if num == 5 {
        "five"
    } else if num == 4 {
        "four"
    } else {
        "other"
    };
    println!("The number is {}", msg);

    // Unconditional Loop
    // It turns out that if the compiler knows a loop is unconditional, it can optimize the code better.
    // loop {
    //     println!("Loop forever!");
    // }
    // break can be used to exit the loop.
    // loop {
    //     println!("Loop forever!");
    //     break; // exit the loop
    // }

    // If we want to exit the nested loop then we can use 'label. Same with continue.
    // 'outer: loop {
    //     println!("Entered the outer loop");
    //     'inner: loop {
    //         println!("Entered the inner loop");
    //         break 'outer; // exit the outer loop
    //     }
    //     println!("This point will never be reached");
    // }

    // While Loop
    // while condition {
    //     // code
    // }
    // While loop are syntactic sugar for loop { if condition { // code } else { break } }
    // There is no do-whie loop in Rust.

    // For Loop
    // for var in expression {
    //     // code
    // }

    for num in [1, 2, 3].iter() {
        println!("{}", num);
    }

    // Functional Programming.
    [1, 2, 3].iter().for_each(|num| println!("{}", num));
    //fold, map, filter, collect, etc. are also available.
    [1, 2, 3]
        .iter()
        .map(|num| num * num)
        .for_each(|num| println!("{}", num));
    [1, 2, 3].iter().fold(0, |acc, num| acc + num);

    let array: [(i32, i32); 3] = [(1, 2), (3, 4), (5, 6)];
    for (x, y) in array.iter() {
        println!("x: {}, y: {}", x, y);
    }

    // Ranges
    // for num in 1..4 {
    //     println!("{}", num);
    // }
    // Start is inclusive and end is exclusive.
    // This will print 1, 2, 3
}
