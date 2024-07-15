  // // while 

  // fn main() {
  //   let mut x = 501; // mut => mutable,   mut keyword is used to make the variable mutable, meaning its value can be changed after it's initialized.
  //   while x > 57 {
  //       x= x / 2;
  //   }
  //   println!("{x}")
  // }




                //   Execution Flow
                // Let's track the value of x in each iteration:

                // Initial Value:

                // x = 501
                // First Iteration:

                // Condition: 501 > 57 (true)
                // x = 501 / 2 = 250 (since integer division truncates the decimal part)
                // Second Iteration:

                // Condition: 250 > 57 (true)
                // x = 250 / 2 = 125
                // Third Iteration:

                // Condition: 125 > 57 (true)
                // x = 125 / 2 = 62
                // Fourth Iteration:

                // Condition: 62 > 57 (true)
                // x = 62 / 2 = 31
                // Fifth Iteration:

                // Condition: 31 > 57 (false)
                // The loop exits because the condition is no longer true.
















//  // for loop


// fn main() {
   
//   for x in 1..5 {        //  1..5 is a range expression that generates values from 1 to 4, inclusive of 1 and exclusive of 5
//     println!("x: {x}")   //  {x} is a placeholder that gets replaced by the current value of x in each iteration.
//   }
//   // 2nd for loop
//   for elem in [1,2,3,4,5,6,7,8,9,10,11] {      //for elem in [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11] means that elem will take each value in the array, one by one, in each iteration.

//     println!("elem is: {elem}")
//   }
// }
















// // loop     =>  The loop statement just loops forever, until a break.



fn main() {
  let mut i =0;     // mut => mutable,   mut keyword is used to make the variable mutable, meaning its value can be changed after it's initialized.

  loop {
    i+= 1;
    println!("i is: {i} ");

    if i > 11 {
      break;
    }
  }
}
