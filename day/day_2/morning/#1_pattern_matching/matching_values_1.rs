// The match keyword lets you match a value against one or more patterns. 
     // The comparisons are done from top to bottom and the first match wins.




     // Rust program uses a match expression to print different messages based on the value of the input character. 
     //      The input character is compared against various patterns, including individual characters, ranges, 
     //      and conditions based on character properties (like is_alphabetic or is_whitespace). 
     //      For each matching case, an appropriate message is printed. If no pattern matches, 
     //      the default case _ prints "Invalid input".






     #[rustfmt::skip] // Skip rustfmt formatting for the following code block
     fn main() {
          
         let input = 'a'; // Declare a variable 'input' with a character 'x'
     
         match input { // Start a match expression to handle different cases for the variable 'input'
             'q'                       =>  println!("quitting the value"), // If 'input' is 'q', print "quitting the value"
             'a' | 's' | 'w' | 'd'     =>   println!("moving around"), // If 'input' is 'a', 's', 'w', or 'd', print "moving around"
             '0'..='9'                 => println!("Number input"), // If 'input' is between '0' and '9', print "Number input"
             key if key.is_alphabetic() => println!("Alphabetic input"), // If 'input' is an alphabetic character, print "Alphabetic input"
             key if key.is_numeric()   => println!("Numeric input"), // If 'input' is a numeric character, print "Numeric input"
             key if key.is_ascii()     => println!("ASCII input"), // If 'input' is an ASCII character, print "ASCII input"
             key if key.is_control()   => println!("Control input"), // If 'input' is a control character, print "Control input"
             key if key.is_whitespace() => println!("Whitespace input"), // If 'input' is a whitespace character, print "Whitespace input"
             key if key.is_ascii_graphic() => println!("Graphic input"), // If 'input' is an ASCII graphic character, print "Graphic input"
             key if key.is_ascii_punctuation() => println!("Punctuation input"), // If 'input' is an ASCII punctuation character, print "Punctuation input"
             key if key.is_ascii_whitespace() => println!("Whitespace input"), // If 'input' is an ASCII whitespace character, print "Whitespace input"
             key if key.is_ascii_control() => println!("Control input"), // If 'input' is an ASCII control character, print "Control input"
             key if key.is_ascii_digit() => println!("Digit input"), // If 'input' is an ASCII digit character, print "Digit input"
             key if key.is_ascii_hexdigit() => println!("Hexdigit input"), // If 'input' is an ASCII hexadecimal digit character, print "Hexdigit input"
             key if key.is_ascii_uppercase() => println!("Uppercase input"), // If 'input' is an ASCII uppercase character, print "Uppercase input"
             key if key.is_ascii_lowercase() => println!("Lowercase input"), // If 'input' is an ASCII lowercase character, print "Lowercase input"
             key if key.is_ascii_alphanumeric() => println!("Alphanumeric input"), // If 'input' is an ASCII alphanumeric character, print "Alphanumeric input"
             key if key.is_ascii_alphabetic() => println!("Alphabetic input"), // If 'input' is an ASCII alphabetic character, print "Alphabetic input"
             key if key.is_lowercase() => println!("Lowercase input: {key}"), // If 'input' is a lowercase character, print "Lowercase input: {key}"
             key if key.is_uppercase() => println!("Uppercase input"), // If 'input' is an uppercase character, print "Uppercase input"
             key if key.is_alphanumeric() => println!("Alphanumeric input"), // If 'input' is an alphanumeric character, print "Alphanumeric input"
             key if key.is_ascii_graphic() => println!("Graphic input"), // If 'input' is a graphic character, print "Graphic input"
             key if key.is_ascii_punctuation() => println!("Punctuation input"), // If 'input' is a punctuation character, print "Punctuation input"
             key if key.is_whitespace() => println!("Whitespace input"), // If 'input' is a whitespace character, print "Whitespace input"
             key if key.is_control() => println!("Control input"), // If 'input' is a control character, print "Control input"
             _                         => println!("Invalid input"), // For any other input, print "Invalid input"
         }
     }
     








    //  The _ pattern is a wildcard pattern which matches any value. The expressions must be exhaustive, 
    //     meaning that it covers every possibility, so _ is often used as the final catch-all case.

    //  Match can be used as an expression. Just like if, each match arm must have the same type. 
    //     The type is the last expression of the block, if any. In the example above, the type is ().
     
    //  A variable in the pattern (key in this example) will create a binding that can be used within the match arm.
     
    //  A match guard causes the arm to match only if the condition is true.
     
     




    // Key Points:

    // You might point out how some specific characters are being used when in a pattern
    
    // | as an or
    // .. can expand as much as it needs to be
    // 1..=5 represents an inclusive range
    // _ is a wild card
    // Match guards as a separate syntax feature are important and necessary when we wish to concisely express 
    //     more complex ideas than patterns alone would allow.
    
    // They are not the same as separate if expression inside of the match arm. 
    //     An if expression inside of the branch block (after =>) happens after the match arm is selected. 
    //     Failing the if condition inside of that block won't result in other arms of the original match expression 
    //     being considered.
    
    // The condition defined in the guard applies to every expression in a pattern with an |.
















// #[rustfmt::skip]
// fn main() {
     
//      let input = 'x';

//      match input {
//           'q'                       =>  println!("quitting the value"),
//           'a' | 's' | 'w' | 'd'     =>   println!("moving around"),
//           '0'..='9'                 => println!("Number input"),
//           key if key.is_alphabetic() => println!("Alphabetic input"),
//           key if key.is_numeric()   => println!("Numeric input"),
//           key if key.is_ascii()     => println!("ASCII input"),
//           key if key.is_control()   => println!("Control input"),
//           key if key.is_whitespace() => println!("Whitespace input"),
//           key if key.is_ascii_graphic() => println!("Graphic input"),
//           key if key.is_ascii_punctuation() => println!("Punctuation input"),
//           key if key.is_ascii_whitespace() => println!("Whitespace input"),
//           key if key.is_ascii_control() => println!("Control input"),
//           key if key.is_ascii_digit() => println!("Digit input"),
//           key if key.is_ascii_hexdigit() => println!("Hexdigit input"),
//           key if key.is_ascii_uppercase() => println!("Uppercase input"),
//           key if key.is_ascii_lowercase() => println!("Lowercase input"),
//           key if key.is_ascii_alphanumeric() => println!("Alphanumeric input"),
//           key if key.is_ascii_alphabetic() => println!("Alphabetic input"),
//           key if key.is_lowercase() => println!("Lowercase input: {key}"),
//           key if key.is_uppercase() => println!("Uppercase input"),
//           key if key.is_alphanumeric() => println!("Alphanumeric input"),
//           key if key.is_ascii_graphic() => println!("Graphic input"),
//           key if key.is_ascii_punctuation() => println!("Punctuation input"),
//           key if key.is_whitespace() => println!("Whitespace input"),
//           key if key.is_control() => println!("Control input"),
//           _                         => println!("Invalid input"),
//      }

// }













// THIS IS SHOWING ERROR , BECAUSE hERE I AM USING ""

// fn main() {
//      let input = "x";
 
//      match input {
//          "q" => println!("quitting the value"),
//          "a" | "s" | "w" | "d" => println!("moving around"),
//          input if input.chars().all(|c| c.is_numeric()) => println!("Number input"),
//          input if input.chars().all(|c| c.is_alphabetic()) => println!("Alphabetic input"),
//          input if input.chars().all(|c| c.is_ascii()) => println!("ASCII input"),
//          input if input.chars().all(|c| c.is_control()) => println!("Control input"),
//          input if input.chars().all(|c| c.is_whitespace()) => println!("Whitespace input"),
//          input if input.is_empty() => println!("Empty input"),
//          input if input.chars().all(|c| c.is_ascii_graphic()) => println!("Graphic input"),
//          input if input.chars().all(|c| c.is_ascii_punctuation()) => println!("Punctuation input"),
//          input if input.chars().all(|c| c.is_ascii_digit()) => println!("Digit input"),
//          input if input.chars().all(|c| c.is_ascii_hexdigit()) => println!("Hexdigit input"),
//          input if input.chars().all(|c| c.is_ascii_uppercase()) => println!("Uppercase input"),
//          input if input.chars().all(|c| c.is_ascii_lowercase()) => println!("Lowercase input"),
//          input if input.chars().all(|c| c.is_ascii_alphanumeric()) => println!("Alphanumeric input"),
//          _ => println!("Invalid input"),
//      }
//  }

 
























