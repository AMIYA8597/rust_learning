// //  macros 


// println!(format, ..) prints a line to standard output, applying formatting described in std::fmt.
// format!(format, ..) works just like println! but returns the result as a string.
// dbg!(expression) logs the value of the expression and returns it.
// todo!() marks a bit of code as not-yet-implemented. If executed, it will panic.
// unreachable!() marks a bit of code as unreachable. If executed, it will panic.





// fn factorial(n: u32) -> u32 {
//     let mut product =1;
//     for i in 1..=n {
//         product*= dbg!(i)
//     }
//     product
// }


// fn fizzbuzz(n: u32) -> u32 {
//     todo!()
//     // println!(n)
// }

// fn main() {
//     let n = 6;
//     println!("{n}! = {}", factorial(n));
// }





fn factorial(n: u32) -> u32 {
    let mut product = 1;
    for i in 1..=n {
        product *= dbg!(i);
    }
    product
}

fn fizzbuzz(n: u32) -> u32 {
    todo!()
}

fn main() {
    let n = 6;
    println!("{n}! = {}", factorial(n));
}













// The takeaway from this section is that these common conveniences exist, and how to use them. Why they are defined as macros, and what they expand to, is not especially critical.

// The course does not cover defining macros, but a later section will describe use of derive macros.