fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &y; // z is an immutable reference to y
    *y = 6; // Modify x through y
    println!("x = {}", x); // Prints 6
    // This line will cause a compile-time error
    // *z = 7;  // error[E0594]: cannot assign to `*z` which is behind a `&` reference
}