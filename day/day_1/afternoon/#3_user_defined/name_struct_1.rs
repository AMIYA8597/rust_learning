// #[derive(Debug)]

struct Person {
    name: String,
    age: u8,
}

fn describe(person: &Person) {
    println!("{} is {} years old", person.name, person.age);
}

fn main(){
    let mut amiya = Person {name:String::from("Amiya"), age:23 };
    // describe(&amiya);

    amiya.age = 24;
    describe(&amiya);

    let name = String::from("Mukesh");
    let age = 27;
    let mukesh = Person {name, age};
    describe(&mukesh);

    let vikash = Person {name: String::from("Vikash"), age: 26, ..mukesh};
    describe(&vikash);

    // println!("{:?}", amiya)
}





















// // Define a struct named `Person` with two fields: `name` and `age`.
// struct Person {
//     name: String,
//     age: u8,
// }

// // Define a function named `describe` that takes a reference to a `Person` and prints their name and age.
// fn describe(person: &Person) {
//     println!("{} is {} years old", person.name, person.age);
// }

// // The main function is the entry point of the program.
// fn main() {
//     // Create a mutable instance of `Person` named `amiya` with name "Amiya" and age 23.
//     let mut amiya = Person { name: String::from("Amiya"), age: 23 };

//     // Call the `describe` function and pass a reference to `amiya`.
//     describe(&amiya); // Output: Amiya is 23 years old

//     // Modify the age of `amiya` to 24.
//     amiya.age = 24;

//     // Call the `describe` function again to see the updated age of `amiya`.
//     describe(&amiya); // Output: Amiya is 24 years old

//     // Create a new instance of `Person` named `mukesh` with name "Mukesh" and age 27.
//     let name = String::from("Mukesh");
//     let age = 27;
//     let mukesh = Person { name, age };

//     // Call the `describe` function and pass a reference to `mukesh`.
//     describe(&mukesh); // Output: Mukesh is 27 years old

//     // Create a new instance of `Person` named `vikash` with name "Vikash", age 26, and use the remaining fields from `mukesh`.
//     let vikash = Person { name: String::from("Vikash"), age: 26, ..mukesh };    //   The syntax     ..mukesh allows us to copy the majority of the fields from the old struct without having to explicitly type it all out. It must always be the last element.

//     // Call the `describe` function and pass a reference to `vikash`.
//     describe(&vikash); // Output: Vikash is 26 years old

//     // Print the `amiya` struct using the `Debug` trait.
//     println!("{:?}", amiya); // Error: `Person` does not implement `Debug`
// }