// Rust has a few control flow constructs which differ from other languages. They are used for pattern matching:

// if let expressions
// let else expressions
// while let expressions


fn main() {
    // println!("Hello, world!")
    let a = 25;
    let b =17;
    if a > b {
        println!("a is greater than b");
    } else {
        println!("a is less than b");
    }
    let x = 5;
    if x == 5 {
        println!("x is five");
    } else {
        println!("x is not five");
    }
    let y = Some(5);
    if let Some(5) = y {
        println!("y is five");
    } else {
        println!("y is not five");
    }
    let z = Some(5);
    if let Some(6) = z {
        println!("z is six");
    } else {
        println!("z is not six");
    }
    let w = Some(5);
    if let Some(i) = w {
        println!("w is {}", i);
    } else {
        println!("w is nothing");
    }
}


