fn main() {
    // Strings
    // There are six types of strings in Rust.
    // 1. &str: String slice. It is a reference to a UTF-8 string.
    // 2. String: Heap-allocated string. It is growable, mutable, owned, UTF-8 encoded string type.
    // 3. OsStr: String slice that is platform specific.
    // 4. OsString: Heap-allocated string that is platform specific.
    // 5. CString: Nul-terminated array of bytes type.
    // 6. CStr: Nul-terminated array of bytes slice type.

    // We care about &str and String.
    // &str is called a string slice. We will see it as borrowed string slice.
    let s: &str = "hello";
    // String slice is a reference to a UTF-8 string.
    // literal string is always a &str.

    // String
    // String is a heap-allocated string. It is growable, mutable, owned, UTF-8 encoded string type.
    // Data is borrowed in &str can not be modified but data in String can be modified.
    let msg = "hello".to_string();
    let mut msg = String::from("hello");

    let s = "hello";
    let str = String::from("world");
    // &str
    // ptr -> "hello"
    // len -> 5
    // Internally made up of some bytes and a length.

    // String
    // ptr -> "world"
    // len -> 5
    // capacity -> 5
    // Internally made up of some bytes, a length, and a capacity.

    // Both string types are valid UTF-8 encoded strings.
    // String can not be indexed by integer because it is a collection of bytes.
    // Because of UTF-8 encoding, it is possible to have multiple bytes for a single character.
    // Thai word "สวัสดี" is 3 characters but 12 bytes.
    // Rust string is a sequence of Unicode scalar values encoded as a stream of UTF-8 bytes.
    let word = "नमस्ते";
    // word[0] is invalid because it is a collection of bytes not characters.
    // To get the first character we can use word.chars().nth(0).unwrap().
    let c = word.chars().nth(0).unwrap();
    println!("{} is the first character in {}", c, word);

    // Graphemes
    // Grapheme is a user-perceived character.
    // In Unicode, a grapheme can be composed of multiple Unicode scalar values.
    // Example: "é" can be represented as a single Unicode scalar value or as two Unicode scalar values.

    // nth() method on chars() iterator returns an Option.
    // Example: "é".chars().nth(0) returns Some('é') and "é".chars().nth(1) returns None.
}
