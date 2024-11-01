fn main() {
    let a = 5; // Default integer type is i32
    println!("The value of a is: {a}");
    let a: u32 = 5; // Now a is u32
    println!("The value of a is: {a}");
    // let b = "42".parse().expect("Not a number!"); // If throws error because the type is unknown
    let b = "42".parse::<i32>().expect("Not a number!"); // This is fine because it has specified type
    println!("The value of b is: {b}");

    // Floats
    let c = 3.14; // Default float type is f64
    println!("The value of c is: {c}");
    let c: f32 = 3.14; // Now c is f32
    println!("The value of c is: {c}");

    // Operators
    let sum = 5 + 10;
    println!("The value of sum is: {sum}");
    // All operators are listed in https://doc.rust-lang.org/book/appendix-02-operators.html

    // Booleans
    let t: bool = true; // Annotation is optional
    println!("The value of t is: {t}");

    // Characters
    let ch: char = '안'; // Single quotes for characters. Size is 4 bytes to handle Unicode
    println!("The value of ch is: {ch}");
}