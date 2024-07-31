// If the field names are unimportant, you can use a tuple struct:

// In Rust, f64 represents a 64-bit floating-point number,  
        // which is commonly used for high-precision floating-point arithmetic. 
        // If you want to implement a function magnitude that calculates the magnitude (or length) of a vector, 
        // you typically use f64 for the result to ensure precision.

// 1 Pound of Force ≈ 4.44822 Newtons

// struct Point(i32, i32);

// fn main() {
//     let p = Point(12,32);
//     println!("{}, {}", p.0, p.1)
// }



// Define a struct for PoundsOfForce
struct PoundsOfForce(f64);

// Define a struct for Newtons
struct Newtons(f64);

// Function to compute the thruster force in PoundsOfForce
fn compute_thruster_force() -> PoundsOfForce {
    // For this example, let's assume a thruster force of 1000 Pounds of Force
    PoundsOfForce(1000.0)
}

// Function to set the thruster force in Newtons
fn set_thruster_force(force: Newtons) {
    println!("Thruster force set to {} Newtons", force.0);
}

// Function to convert PoundsOfForce to Newtons
fn pounds_to_newtons(pounds: PoundsOfForce) -> Newtons {
    // Convert PoundsOfForce to Newtons using the conversion factor 1 lbf ≈ 4.44822 N
    Newtons(pounds.0 * 4.44822)
}

// Main function
fn main() {
    // Compute the thruster force in PoundsOfForce
    let force_pounds = compute_thruster_force();

    // Convert the thruster force to Newtons
    let force_newtons = pounds_to_newtons(force_pounds);

    // Set the thruster force using the value in Newtons
    set_thruster_force(force_newtons);
}

















// Newtypes are a great way to encode additional information about the value in a primitive type, for example:
// The number is measured in some units: Newtons in the example above.
// The value passed some validation when it was created, so you no longer have to validate 
        // it again at every use: PhoneNumber(String) or OddNumber(u32).
// Demonstrate how to add a f64 value to a Newtons type by accessing the single field in the newtype.
// Rust generally doesn’t like inexplicit things, like automatic unwrapping or for instance using booleans as integers.
// Operator overloading is discussed on Day 3 (generics).
// The example is a subtle reference to the Mars Climate Orbiter failure.