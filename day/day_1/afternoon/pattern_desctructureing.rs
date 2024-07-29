fn pattern_tupple(tuple: (i32,i32,i32,i32)) {
    let first = tuple.0;
    let second = tuple.1;
    let third = tuple.2;
    let fourth = tuple.3;
    println!("first: {first}, second: {second}, third: {third}, fourth: {fourth}")
}


fn main() {
    let mutex = (45,68,93,68);
    pattern_tupple(mutex);
}


// fn main() {
//     println!("hello ");
//     println!("hello ");
//     println!("hello ");
//     println!("hello ");
//     println!("hello ");
//     println!("hello ");
//     // println!("{}", pattern_tupple("first: {first}, second: {second}, third: {third}, fourth: {fourth}"))
//     // println!("{}", pattern_tupple("first: {first}, second: {second}, third: {third}, fourth: {fourth}", first, second, third, fourth))
// }
















// The patterns used here are “irrefutable”, meaning that the compiler can statically verify that the value on the right of = has the same structure as the pattern.
// A variable name is an irrefutable pattern that always matches any value, hence why we can also use let to declare a single variable.
// Rust also supports using patterns in conditionals, allowing for equality comparison and destructuring to happen at the same time. This form of pattern matching will be discussed in more detail later.
// Edit the examples above to show the compiler error when the pattern doesn’t match the value being matched on.