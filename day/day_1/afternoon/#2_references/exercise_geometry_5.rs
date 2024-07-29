// // Calculate the magnitude of a vector by summing the squares of its coordinates
// // and taking the square root. Use the `sqrt()` method to calculate the square
// // root, like `v.sqrt()`.


// fn magnitude(...) -> f64 {
//     todo!()
// }

// // Normalize a vector by calculating its magnitude and dividing all of its
// // coordinates by that magnitude.


// fn normalize(...) {
//     todo!()
// }

// // Use the following `main` to test your work.

// fn main() {
//     println!("Magnitude of a unit vector: {}", magnitude(&[0.0, 1.0, 0.0]));

//     let mut v = [1.0, 2.0, 9.0];
//     println!("Magnitude of {v:?}: {}", magnitude(&v));
//     normalize(&mut v);
//     println!("Magnitude of {v:?} after normalization: {}", magnitude(&v));
// }














// /// Calculate the magnitude of the given vector.
// fn magnitude(vector: &[f64; 3]) -> f64 {
//     let mut mag_squared = 0.0;
//     for coord in vector {
//         mag_squared += coord * coord;
//     }
//     mag_squared.sqrt()
// }

// /// Change the magnitude of the vector to 1.0 without changing its direction.
// fn normalize(vector: &mut [f64; 3]) {
//     let mag = magnitude(vector);
//     for item in vector {
//         *item /= mag;
//     }
// }

// fn main() {
//     println!("Magnitude of a unit vector: {}", magnitude(&[0.0, 1.0, 0.0]));

//     let mut v = [1.0, 2.0, 9.0];
//     println!("Magnitude of {v:?}: {}", magnitude(&v));
//     normalize(&mut v);
//     println!("Magnitude of {v:?} after normalization: {}", magnitude(&v));
// }


















/// Calculate the magnitude of the given vector.
fn magnitude(vector: &[f64; 3]) -> f64 {         // In Rust, f64 represents a 64-bit floating-point number, which is commonly used for high-precision floating-point arithmetic. If you want to implement a function magnitude that calculates the magnitude (or length) of a vector, you typically use f64 for the result to ensure precision.
    // Initialize a variable to hold the sum of the squares of the coordinates.
    let mut mag_squared = 0.0;
    // Iterate over each coordinate in the vector.
    for coord in vector {
        // Add the square of the current coordinate to the sum.
        mag_squared += coord * coord;
    }
    // Return the square root of the sum of squares, which is the magnitude.
    mag_squared.sqrt()
}

/// Change the magnitude of the vector to 1.0 without changing its direction.
fn normalize(vector: &mut [f64; 3]) {
    // Calculate the magnitude of the vector.
    let mag = magnitude(vector);
    // Iterate over each coordinate in the vector.
    for item in vector {
        // Divide each coordinate by the magnitude to normalize the vector.
        *item /= mag;
    }
}

fn main() {
    // Calculate and print the magnitude of a unit vector.
    println!("Magnitude of a unit vector: {}", magnitude(&[0.0, 1.0, 0.0]));

    // Define a mutable vector.
    let mut v = [1.0, 2.0, 9.0];
    // Calculate and print the magnitude of the vector `v`.
    println!("Magnitude of {v:?}: {}", magnitude(&v));
    // Normalize the vector `v`.
    normalize(&mut v);
    // Calculate and print the magnitude of the vector `v` after normalization.
    println!("Magnitude of {v:?} after normalization: {}", magnitude(&v));
}
