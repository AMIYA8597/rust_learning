// Static variables will live during the whole execution of the program, and therefore will not move:






static BANNER: &str = "Welcome to Rust static part";

fn main() {
    println!("{BANNER}")
}







// As noted in the Rust RFC Book, these are not inlined upon use and have an actual associated memory location. 
//  This is useful for unsafe and embedded code, and the variable lives through the entirety of the program execution.
//  When a globally-scoped value does not have a reason to need object identity, const is generally preferred.









// static is similar to mutable global variables in C++.
// static provides object identity: an address in memory and state as required by types with interior mutability such as Mutex<T>.
// More to Explore
// Because static variables are accessible from any thread, they must be Sync. Interior mutability is possible through a Mutex, atomic or similar.

// Thread-local data can be created with the macro std::thread_local.