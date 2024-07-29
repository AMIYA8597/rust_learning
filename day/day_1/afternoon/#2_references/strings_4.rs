// We can now understand the two string types in Rust:

// &str is a slice of UTF-8 encoded bytes, similar to &[u8].
// String is an owned buffer of UTF-8 encoded bytes, similar to Vec<T>.





fn main() {
    let s1: &str = " Why Solana used you as a chainCode?";  // Define an immutable string slice `s1` and initialize it with the value.
    println!("s1: {s1}");

    let mut s2: String = String::from("Hello Rust");  // This line defines a mutable string s2 of type String and initializes it with the value "Hello ".
    println!("1st s2: {s2}");
    s2.push_str(s1);
    println!("2nd s2: {s2}");

    let s3: &str = &s2[s2.len() - s1.len()..];  // This line creates an immutable string slice s3 that references the last part of s2, which is the same length as s1.      s2[s2.len() - s1.len()..] computes a slice of s2 starting from the index 
    println!("s3: {s3}");
}













// &str introduces a string slice, which is an immutable reference to UTF-8 encoded string data stored in a block of memory. String literals ("Hello"), are stored in the program’s binary.

// Rust's String type is a wrapper around a vector of bytes. As with a Vec<T>, it is owned.

// As with many other types String::from() creates a string from a string literal; String::new() creates a new empty string, to which string data can be added using the push() and push_str() methods.

// The format!() macro is a convenient way to generate an owned string from dynamic values. It accepts the same format specification as println!().

// You can borrow &str slices from String via & and optionally range selection. If you select a byte range that is not aligned to character boundaries, the expression will panic. The chars iterator iterates over characters and is preferred over trying to get character boundaries right.

// For C++ programmers: think of &str as std::string_view from C++, but the one that always points to a valid string in memory. Rust String is a rough equivalent of std::string from C++ (main difference: it can only contain UTF-8 encoded bytes and will never use a small-string optimization).

// Byte strings literals allow you to create a &[u8] value directly:

// Raw strings allow you to create a &str value with escapes disabled: r"\n" == "\\n". You can embed double-quotes by using an equal amount of # on either side of the quotes:










// str:
    // Immutable.
    // Usually borrowed as &str.
    // Does not own its data.
    // More efficient for static strings or slices.
// String:
    // Mutable.
    // Owns its data.
    // Heap-allocated.
    // Useful for dynamic string manipulation.