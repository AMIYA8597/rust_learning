// Greatest Common Divisor (GCD)



fn demo(a: u32, b:u32) -> u32 {
    if b > 0 {
        demo(b , a % b)
    } else {
        a
    }
}

fn main() {
    println!("demo is: {}", demo(186,17))
}







// Declaration parameters are followed by a type (the reverse of some programming languages), then a return type.
// The last expression in a function body (or any block) becomes the return value. Simply omit the ; at the end of the expression. The return keyword can be used for early return, but the “bare value” form is idiomatic at the end of a function (refactor gcd to use a return).
// Some functions have no return value, and return the ‘unit type’, (). The compiler will infer this if the -> () return type is omitted.
// Overloading is not supported – each function has a single implementation.
// Always takes a fixed number of parameters. Default arguments are not supported. Macros can be used to support variadic functions.
// Always takes a single set of parameter types. These types can be generic, which will be covered later.


    










// The Greatest Common Divisor (GCD) of two integers is the largest positive integer that divides both numbers without leaving a remainder. The GCD of two numbers \( a \) and \( b \) can be found using the Euclidean algorithm, which is based on the principle that the GCD of two numbers also divides their difference.

// ### Euclidean Algorithm Steps:

// 1. **Divide** \( a \) by \( b \) and find the remainder \( r \).
// 2. **Replace** \( a \) with \( b \) and \( b \) with \( r \).
// 3. **Repeat** steps 1 and 2 until \( b \) becomes 0.
// 4. The GCD is the last non-zero value of \( b \).

// ### Example:

// To find the GCD of 186 and 17:
// 1. \( 186 \mod 17 = 16 \) (remainder is 16)
// 2. \( 17 \mod 16 = 1 \) (remainder is 1)
// 3. \( 16 \mod 1 = 0 \) (remainder is 0)

// When \( b \) becomes 0, the GCD is the last non-zero remainder, which is 1 in this case. Therefore, the GCD of 186 and 17 is 1.

// ### Properties of GCD:

// - The GCD of two numbers also divides their sum and difference.
// - The GCD is used in simplifying fractions, finding least common multiples, and in various applications in number theory and cryptography.

// ### Mathematically:

// \[ \text{GCD}(a, b) = \text{GCD}(b, a \mod b) \]

// Continue this process until \( b \) is zero. The non-zero remainder at that step is the GCD.






