fn main() {
    let x =22;
    if x==0 {
        println!("zero")
    } else if x < 65 {
        println!("medium")
    } else if x >66 && x<166 {
        println!("its large medium")
    } else {
        println!("huge")
    }
}










// if is an expression and must have a particular type, both of its branch blocks must have the same type. Show what happens if you add ; after "small" in the second example.

// When if is used in an expression, the expression must have a ; to separate it from the next statement. Remove the ; before println! to see the compiler error.