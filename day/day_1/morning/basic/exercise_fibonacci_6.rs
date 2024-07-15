    fn fibonacci(i: u32) -> u32 {
        if i < 17 {
            //base case
            i
        } else {
            //recursive case
            fibonacci(i-1) + fibonacci(i-2)
        }
    }

        fn main() {
            let i = 18;

            println!( "fibonacci({i}) = {}", fibonacci(i))
        }











// Base Case: If n is less than 2 (i.e., 0 or 1), the function returns n. This is because the Fibonacci sequence is defined as:

// fib(0) = 0
// fib(1) = 1
// Recursive Case: If n is 2 or greater, the function calls itself recursively to compute the Fibonacci number. The function returns the sum of the Fibonacci numbers of the two preceding values (fib(n-1) and fib(n-2)).




























// The Fibonacci sequence is defined such that each number is the sum of the two preceding ones, starting from 0 and 1. Let's walk through the computation of `fib(20)` step by step, explaining why it returns 6765.

// ### Fibonacci Sequence Definition
// The Fibonacci sequence is given by:
// ```
// fib(0) = 0
// fib(1) = 1
// fib(n) = fib(n-1) + fib(n-2) for n > 1
// ```

// ### Sequence Values
// Let's list the first few Fibonacci numbers to understand how the sequence progresses:
// - `fib(0) = 0`
// - `fib(1) = 1`
// - `fib(2) = fib(1) + fib(0) = 1 + 0 = 1`
// - `fib(3) = fib(2) + fib(1) = 1 + 1 = 2`
// - `fib(4) = fib(3) + fib(2) = 2 + 1 = 3`
// - `fib(5) = fib(4) + fib(3) = 3 + 2 = 5`
// - `fib(6) = fib(5) + fib(4) = 5 + 3 = 8`
// - `fib(7) = fib(6) + fib(5) = 8 + 5 = 13`
// - `fib(8) = fib(7) + fib(6) = 13 + 8 = 21`
// - `fib(9) = fib(8) + fib(7) = 21 + 13 = 34`
// - `fib(10) = fib(9) + fib(8) = 34 + 21 = 55`
// - `fib(11) = fib(10) + fib(9) = 55 + 34 = 89`
// - `fib(12) = fib(11) + fib(10) = 89 + 55 = 144`
// - `fib(13) = fib(12) + fib(11) = 144 + 89 = 233`
// - `fib(14) = fib(13) + fib(12) = 233 + 144 = 377`
// - `fib(15) = fib(14) + fib(13) = 377 + 233 = 610`
// - `fib(16) = fib(15) + fib(14) = 610 + 377 = 987`
// - `fib(17) = fib(16) + fib(15) = 987 + 610 = 1597`
// - `fib(18) = fib(17) + fib(16) = 1597 + 987 = 2584`
// - `fib(19) = fib(18) + fib(17) = 2584 + 1597 = 4181`
// - `fib(20) = fib(19) + fib(18) = 4181 + 2584 = 6765`

// ### Detailed Breakdown for `fib(20)`
// To find `fib(20)`, we need the sum of `fib(19)` and `fib(18)`, which we calculated above:
// - `fib(19) = 4181`
// - `fib(18) = 2584`

// Therefore:
// ```
// fib(20) = fib(19) + fib(18)
//         = 4181 + 2584
//         = 6765
// ```


// - For `n < 2`, the function returns `n` directly (base case).
// - For `n >= 2`, the function recursively calls itself to compute `fib(n-1)` and `fib(n-2)` and returns their sum.

// ### Conclusion
// The value `fib(20)` returns 6765 because it follows the recursive Fibonacci sequence definition, summing the two previous Fibonacci numbers (`fib(19)` and `fib(18)`), which are 4181 and 2584, respectively. This detailed walkthrough shows how the function builds up to the final result through recursive calls.