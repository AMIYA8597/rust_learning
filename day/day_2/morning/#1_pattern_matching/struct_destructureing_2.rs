// Like tuples, Struct can also be destructured by matching:


struct Foo {
    x: (u32, u64, u32), // Define a struct Foo with two fields x and y, each being a tuple of (u32, u64, u32)
    y: (u32, u64, u32),
}

#[rustfmt::skip] // Skip rustfmt formatting for the following code block

fn main() {
    let foo = Foo { x: (54,42,62), y: (36, 85,29)}; // Initialize a Foo struct with specific values for x and y

    match foo { // Start a match expression to handle different patterns for the foo variable
        Foo { x: (1, b, 5), y: (8, 6, h) } => println!("First"), // If foo.x matches (1, b, 5) and foo.y matches (8, 6, h), print "First"
        Foo { x: (a, b, c), y: (d, e, f) } if c == f => println!("Second"), // If c is equal to f, print "Second"
        Foo { x: (a, b, c), y: (d, e, f) } if c != f => println!("Third"), // If c is not equal to f, print "Third"
        _ => println!("Fourth"), // For any other pattern, print "Fourth"
    }
}













// struct Foo {
//     x: (u32, u64, u32),
//     y: (u32, u64, u32),
// }

// #[rustfmt::skip]

// fn main() {
//     let foo = Foo { x: (54,42,62), y: (36, 85,29)};

//     match foo {
//         Foo {x : (1,b,5 ), y: (8,6,h) } => println!("First"),
//         Foo {x: (a, b, c), y: (d, e, f)} if c == f => println!("Second"),
//         Foo {x: (a, b, c), y: (d, e, f)} if c != f => println!("Third"),
//         _ => println!("Fourth"),
//     }
// }




// This Rust program defines a struct Foo with two tuple fields, x and y. 
//     It then creates an instance of Foo and uses a match expression to pattern match on the fields of foo. 
//     Different cases are provided to match specific patterns in foo's fields. The patterns include:

// Matching specific values.
// Matching based on a condition (if guard).
// A default case that handles any other pattern not explicitly covered by previous cases.

// Only the cases that are enabled will execute, with the program printing different messages depending on which pattern 
//     matches. The commented-out lines show additional patterns that can be used to match specific parts 
//     of the Foo struct.





// Change the literal values in foo to match with the other patterns.
// Add a new field to Foo and make changes to the pattern as needed.
// The distinction between a capture and a constant expression can be hard to spot. 
//     Try changing the 2 in the second arm to a variable, and see that it subtly doesn't work. 
//     Change it to a const and see it working again.