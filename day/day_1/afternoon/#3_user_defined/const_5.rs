// Constants are evaluated at compile time and their values are inlined wherever they are used:






// Define a constant for the size of the digest array
const DIGEST_SIZE: usize = 3;

// Define a constant Option value that holds a u8 value
const ZERO: Option<u8> = Some(42);

// Function to compute a simple digest for a given text
fn compute_digest(text: &str) -> [u8; DIGEST_SIZE] {
    // Create a mutable array `digest` with the size of `DIGEST_SIZE`, 
    // initialized with the value in `ZERO` or 0 if `ZERO` is `None`
    let mut digest = [ZERO.unwrap_or(0); DIGEST_SIZE];

    // Iterate over the bytes of the input text
    for (idx, &b) in text.as_bytes().iter().enumerate() {
        // Update the digest by adding the byte value to the corresponding position
        digest[idx % DIGEST_SIZE] = digest[idx % DIGEST_SIZE].wrapping_add(b);
    }

    // Return the computed digest
    digest
}

// Main function, the entry point of the program
fn main() {
    // Compute the digest for the text "Hello"
    let digest = compute_digest("Hello");

    // Print the computed digest
    println!("digest: {digest:?}");
}




















// Mention that const behaves semantically similar to C++'s constexpr
// It isn't super common that one would need a runtime evaluated constant, 
//     but it is helpful and safer than using a static.