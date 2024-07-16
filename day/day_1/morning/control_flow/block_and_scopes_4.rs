
// // blocks and scopes 

// fn main() {
//     let x = 10;
//     let y = {
//         let z = 8;
//         println!(" z is : {z}");
//         x - z
//     };
//     println!(" y is : {y}");
// }







// ou can show how the value of the block changes by changing the last line in the block. For instance, adding/removing a semicolon or using a return.














// //  scope and shadowing





fn main() {
    let x = 11;
    println!(" before starting bracket: {x}");

    {
        let x = "remaining";
        println!("block scope : {x}");

        let x = true;
        println!("shadowed in block scope : {x}");
    }

    println!("after completeing block scope : {x}")
}







// Show that a variable’s scope is limited by adding a b in the inner block in the last example, and then trying to access it outside that block.
// Shadowing is different from mutation, because after shadowing both variable’s memory locations exist at the same time. Both are available under the same name, depending where you use it in the code.
// A shadowing variable can have a different type.
// Shadowing looks obscure at first, but is convenient for holding on to values after .unwrap().





