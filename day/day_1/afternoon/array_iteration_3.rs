fn main() {
    let primes = [2,3,5,6,7,9,10,12,13,14,15,17,18];
    for prime in primes {
        for i in 2..prime {
            assert_ne!(prime % i, 0 )   //In Rust, the macro assert_ne! is used to assert that two expressions are not equal to each other. If the expressions are equal, the macro will cause the program to panic and terminate execution. It is part of Rust's standard library and is typically used for debugging and testing to ensure that certain conditions hold true during program execution.
        }
    }
}













// This functionality uses the IntoIterator trait, but we haven’t covered that yet.

// The assert_ne! macro is new here. There are also assert_eq! and assert! macros. These are always checked while, debug-only variants like debug_assert! compile to nothing in release builds.