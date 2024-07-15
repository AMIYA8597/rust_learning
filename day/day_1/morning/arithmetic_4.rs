fn arithmetic(a: i32, b:i32, c:i32) ->i32 {
    return a * b + b * c + c * a;
}

fn main() {
    println!(" result is {}",  arithmetic(281,653,749))
}







// This is the first time we’ve seen a function other than main, but the meaning should be clear: it takes three integers, and returns an integer. Functions will be covered in more detail later.

// Arithmetic is very similar to other languages, with similar precedence.

// What about integer overflow? In C and C++ overflow of signed integers is actually undefined, and might do unknown things at runtime. In Rust, it’s defined.

// Change the i32’s to i16 to see an integer overflow, which panics (checked) in a debug build and wraps in a release build. There are other options, such as overflowing, saturating, and carrying. These are accessed with method syntax, e.g., (a * b).saturating_add(b * c).saturating_add(c * a).

// In fact, the compiler will detect overflow of constant expressions, which is why the example requires a separate function.