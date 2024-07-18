// // Arrays 


fn main() {
    let mut a: [i8; 10] = [42; 10];      //it's an array of 10 elements where each element is of type i8 (which is a signed 8-bit integer). The array is initialized with the value 42 repeated 10 times.
    a[5] = 0;               // Arrays in Rust are zero-indexed, so a[5] refers to the sixth element (index 5) of the array
    println!("a: {:?}", a)
}





// So, if we analyze what happens step by step:

// Initially, a is [42, 42, 42, 42, 42, 42, 42, 42, 42, 42].
// After a[5] = 0;, a becomes [42, 42, 42, 42, 42, 0, 42, 42, 42, 42].
// The println! statement then prints a: [42, 42, 42, 42, 42, 0, 42, 42, 42, 42] to the console.
