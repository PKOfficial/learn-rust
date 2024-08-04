fn main() {
    // Scalar types
    // There are four primary scalar types in Rust: integers, floating-point numbers, Booleans, and characters.

    // Integers
    // Unsigned integers: u8, u16, u32, u64, u128, usize
    // Starts with u and followed by the number of bits they take up in memory.
    // Except usize, usize is the size of a pointer on the system.
    // This is also the type we will usually use to index into an array or vector.

    // Signed integers: i8, i16, i32, i64, i128, isize

    // Integer literals
    // Integer literals can be expressed in decimal, hexadecimal, octal, binary, and byte (u8 only).
    // Decimal: 98_222, 1000000 is the same as 1_000_000
    // Hex: 0xff
    // Octal: 0o77
    // Binary: 0b1111_0000
    // Byte (u8 only): b'A'

    // Floating-point numbers
    // f32, f64
    // f32 has 32 bits of precision, f64 has 64 bits of precision.
    // f64 is the default type for floating-point numbers because on modern CPUs it's roughly the same speed as f32 but is capable of more precision.
    // f64 can be really slow on less than 64-bit architectures.
    // Floating point literals follows IEEE-754 standard.
    // 1.0, 1.0f64, 1.0f32
    // .1 is not a valid float in Rust. 0.1 is a valid float.
    // let x = 5u16; // x is an unsigned 16-bit integer
    // let y = 3.14_f64; // y is a 64-bit floating point number

    // Booleans
    // bool
    // true, false (Two boolean literals)

    // Characters types
    // char
    // char literals are specified with single quotes, as opposed to string literals, which use double quotes.
    // Rust’s char type is four bytes in size and represents a Unicode Scalar Value.
    // let c = 'z';
    // let z = 'ℤ';
    // let heart_eyed_cat = '😻';
    // UCS-4 (4 bytes) is used to represent a character in Rust.

    // Strings are UTF-8 and characters are not, so string do not use character internally.
    // Source file encoding is UTF-8 by default.
    // Mostly when we want to deal with single character it is going to be UTF-8 string not character literal
}