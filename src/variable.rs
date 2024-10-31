fn main() {
    // Immutable variable
    let a = 5;
    println!("The value of a is: {a}");
    // a = 6; // This will cause an error, because a is immutable

    // Mutable variable
    let mut b = 1;
    println!("The value of b is: {b}");
    b = 2; // now it is fine, because b is mutable
    println!("The value of b is: {b}");

    // Constants
    const CONST: u32 = 3;  // Constants always have data type annotations
    println!("The value of CONST is: {CONST}");

    // Shadowing
    let c = 4;
    println!("The value of c is: {c}");
    let c = c + 1; // Now c is immutable, but this is allowed because we are creating a new variable with the same name
    println!("The value of c is: {c}");

    let d = 5;
    println!("The value of outer d is: {d}");
    {
        let d = 7; // This is a new variable, not the same as the outer d
        println!("The value of inner d is: {d}");
    }
    println!("The value of outer d is: {d}"); // This d is 5 again, because the inner d is a different variable

    let mut e = "hello"; // The type is string
    println!("The value of e is: {e}");
    // e = e.size(); // This will cause an error, because e was string but assigning int
    let e = e.len(); // Now it is fine because we are creating a new variable with the same name
    println!("The value of e is: {e}");
}