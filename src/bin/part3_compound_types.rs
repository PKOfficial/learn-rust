fn main() {
    // Compound types
    // Compound types gather multiple values of other types into one type.
    // Rust has two primitive compound types: tuples and arrays.

    // Tuples
    // A tuple is a general way of grouping together a number of values with a variety of types into one compound type.
    // Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    // let tup = (500, 6.4, 1);
    // let first_element = tup.0;
    // let second_element = tup.1;
    // let third_element = tup.2;
    // let (first_element, second_element, third_element) = tup;
    // Arity of a tuple is the number of elements in the tuple.
    // let tup = (500, 6.4, 1); // Tuple with arity 3
    // Current maximum arity of a tuple is 12.

    // Arrays
    // An array is a collection of elements of the same type.
    // let arr = [1, 2, 3, 4, 5];
    // let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // arr[0] // 1
    // array lives on stack by default.
    // Arrays have fixed length: once declared, they cannot grow or shrink in size.
}
