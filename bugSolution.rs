fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    // Create a mutable reference to x
    let z = &mut x;
    *z = 7; // Modify x through z
    println!("x = {}", x); // Prints 7
    }
