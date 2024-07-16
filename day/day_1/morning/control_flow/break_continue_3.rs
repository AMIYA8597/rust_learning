// // break & continue


            // fn main() {
            //     let mut i = 0;  // mut => mutable,   mut keyword is used to make the variable mutable, meaning its value can be changed after it's initialized.
            //     loop {
            //         i += 1;
            //         if i > 7 {
            //             break;
            //         }
            //         if i % 2 ==0 {
            //             continue;
            //         }

            //         // println!(" i is :", i);    // error  => println!(" {}", i);
            //         println!(" {}", i);
            //     }
            // }








// // labels


fn main() {
    // let array_2d = [ [1,2,4], [5,6,8], [9,10,12], [13,14,16]];
    let array_2d = [ [1,2,4,7], [5,6,8,11], [9,10,12,15], [13,14,16,19]];
    
    let mut searched_array = 0;           // mut => mutable,   mut keyword is used to make the variable mutable, meaning its value can be changed after it's initialized.
    let targeted_value= 15;

    // 'outer: for i in 0..=2 {
        'outer: for i in 0..=3 {
                // for j in 0..=2 {
                    for j in 0..=3 {
                    searched_array +=1;
                    if array_2d[i][j]== targeted_value {
                        break 'outer;
                    }
                }
    }
    println!("searched_array: {searched_array}")
}

















// fn main() {
//     let s = [[5, 6, 7], [8, 9, 10], [21, 15, 32]];
//     let mut elements_searched = 0;
//     let target_value = 10;
//     'outer: for i in 0..=2 {
//         for j in 0..=2 {
//             elements_searched += 1;
//             if s[i][j] == target_value {
//                 break 'outer;
//             }
//         }
//     }
//     print!("elements searched: {elements_searched}");
// }