// Define an enum called Result with two variants: Ok and Error
// Ok holds an i32 value, and Error holds a String value
enum Result {
    Ok(i32),
    Error(String),
}

// Define a function named divide that takes an i32 as input and returns a Result
fn divide(n: i32) -> Result {
    // Check if the input number n is even
    if n % 2 == 0 {
        // If n is even, return the Ok variant of Result with n divided by 2
        Result::Ok(n / 2)
    } else {
        // If n is odd, return the Error variant of Result with a formatted error message
        Result::Error(format!("cannot divide {n} into the equal part"))
    }
}

// Define the main function, which is the entry point of the program
fn main() {
    // Assign the value 998 to the variable n
    let n = 998;
    
    // Call the divide function with n and pattern match on the result
    match divide(n) {
        // If the result is Ok, extract the value and print the success message
        Result::Ok(half) => println!("{n} divided by two is {half}"),
        
        // If the result is Error, extract the error message and print the error message
        Result::Error(msg) => println!("sorry there is an error: {msg}"),
    }
}







// The if/else expression is returning an enum that is later unpacked with a match.

// You can try adding a third variant to the enum definition and displaying the errors when running the code. 
//     Point out the places where your code is now inexhaustive and how the compiler tries to give you hints.

// The values in the enum variants can only be accessed after being pattern matched.

// Demonstrate what happens when the search is inexhaustive. Note the advantage the Rust compiler provides by confirming 
//     when all cases are handled.

// Save the result of divide_in_two in the result variable and match it in a loop. 
//     That won't compile because msg is consumed when matched. To fix it, match &result instead of result. 
//     That will make msg a reference so it won't be consumed. This "match ergonomics" appeared in Rust 2018. 
//     If you want to support older Rust, replace msg with ref msg in the pattern.











// enum Result {
//     Ok(i32),
//     Error(String),
// }

// fn divide(n:i32) -> Result {
//     if n % 2 == 0 {
//         Result::Ok(n / 2)
//     } 
//     else {
//         Result::Error(format!("cannot divide {n} into the equal part"))
//     }
// }

// fn main() {
//     let n =998;
//     match divide(n) {
//         Result::Ok(half) => println!("{n} divided by two is {half}"),
//         Result::Error(msg) => println!("sorry there is an error: {msg}")
//     }
// }
































// fn divide(a: i32, b: i32) -> Result {
//     if b == 0 {
//         Err("Division by zero".to_string())
//     } else {
//         Ok(a / b)
//     }
// }
// fn main() {
//     let result = divide(10, 2);
//     match result {
//         Ok(value) => println!("Result: {}", value),
//         Err(error) => println!("Error: {}", error),
//     }

//     let result = divide(10, 0);
//     match result {
//         Ok(value) => println!("Result: {}", value),
//         Err(error) => println!("Error: {}", error),
//     }
// }



// fn_pointer_2.rsfn add(a: i32, b: i32) -> i32 {
//     a + b
// }
// fn_pointer_2.rsfn subtract(a: i32, b: i32) -> i32 {
//     a - b
// }
// fn_pointer_2.rsfn multiply(a: i32, b: i32) -> i32 {
//     a * b
// }
// fn_pointer_2.rsfn divide(a: i32, b: i32) -> i32 {
//     a / b
// }
// fn_pointer_2.rsfn main() {
//     let operations = [add, subtract, multiply, divide];
//     let a = 10;
//     let b = 5;
//     for operation in operations {
//         let result = operation(a, b);
//         println!("Result: {}", result);
//     }
// }
// fn_pointer_2.rsfn add(a: i32, b: i32) -> i32 {
//     a + b
// }
// fn_pointer_2.rsfn subtract(a: i32, b: i32) -> i32 {
//     a - b
// }
// fn_pointer_2.rsfn multiply(a: i32, b: i32) -> i32 {
//     a * b
// }
// fn_pointer_2.rsfn divide(a: i32, b: i32) -> i32 {
//     a / b
// }
// fn_pointer_2.rsfn main() {
//     let operations = [add, subtract, multiply, divide];
//     let a = 10;
//     let b = 5;
//     for operation in operations {
//         let result = operation(a, b);
//         println!("Result: {}", result);
//     }
// }
