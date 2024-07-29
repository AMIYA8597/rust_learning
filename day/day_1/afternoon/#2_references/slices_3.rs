fn main() {
    let a: [i32;7] = [54,76,68,24,65,23,81];
    println!("a: {a:?}");       // Print the entire array `a` using the debug format specifier `:?`.

    let s: &[i32] = &a[1..5];   // Create an immutable slice `s` that includes elements from index 1 to index 5 (inclusive) of the array `a`.

    println!("s: {s:?}");       // Print the slice `s` using the debug format specifier `:?`.
}















// We create a slice by borrowing a and specifying the starting and ending indexes in brackets.

// If the slice starts at index 0, Rust’s range syntax allows us to drop the starting index, meaning that &a[0..a.len()] and &a[..a.len()] are identical.

// The same is true for the last index, so &a[2..a.len()] and &a[2..] are identical.

// To easily create a slice of the full array, we can therefore use &a[..].

// s is a reference to a slice of i32s. Notice that the type of s (&[i32]) no longer mentions the array length. This allows us to perform computation on slices of different sizes.

// Slices always borrow from another object. In this example, a has to remain 'alive' (in scope) for at least as long as our slice.

// The question about modifying a[3] can spark an interesting discussion, but the answer is that for memory safety reasons you cannot do it through a at this point in the execution, but you can read the data from both a and s safely. It works before you created the slice, and again after the println, when the slice is no longer used.