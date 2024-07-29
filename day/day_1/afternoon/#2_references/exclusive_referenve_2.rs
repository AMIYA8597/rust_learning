fn main() {
    let mut point = (3,7);
    let co_ordinator = &mut point.0;     // point.0 accesses the first element of the tuple (which is 1).       &mut point.0 creates a mutable reference to this element and assigns it to co_ordinator.
    *co_ordinator = 33;  //This line dereferences the mutable reference co_ordinator using the * operator and assigns the value 33 to it.
    println!("point: {point:?}")

}








// "Exclusive" means that only this reference can be used to access the value. No other references (shared or exclusive) can exist at the same time, and the referenced value cannot be accessed while the exclusive reference exists. Try making an &point.0 or changing point.0 while x_coord is alive.

// Be sure to note the difference between let mut x_coord: &i32 and let x_coord: &mut i32. The first one represents a shared reference which can be bound to different values, while the second represents an exclusive reference to a mutable value.




