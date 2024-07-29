fn print_tuple(tuple: (i32, i32)) {
    let left = tuple.0;
    let right = tuple.1;

    println!("left: {left}, right: {right}")
}

// // // //  Rust also supports using pattern matching to destructure a larger value into its constituent parts:

// fn print_tuple(tuple: (i32, i32)) {
//     let (left, right) = tuple;

//     println!("left: {left}, right: {right}")
// }

fn main() {
    let tuple = (45,67);
    print_tuple(tuple);
}














    // The patterns used here are "irrefutable", meaning that the compiler can statically verify that the value on the right of = has the same structure as the pattern.
    // A variable name is an irrefutable pattern that always matches any value, hence why we can also use let to declare a single variable.
    // Rust also supports using patterns in conditionals, allowing for equality comparison and destructuring to happen at the same time. This form of pattern matching will be discussed in more detail later.
    // Edit the examples above to show the compiler error when the pattern doesn't match the value being matched on.