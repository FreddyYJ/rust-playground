fn main() {
    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1); // Annotation is optional
    let first = tup.0; // Accessing tuple elements
    println!("The value of first is: {first}");
    let (first, second, third) = tup; // Destructuring
    println!("The value of second is: {second}");

    // Arrays
    let arr: [u32; 5] = [1, 2, 3, 4, 5]; // Every elements must have the same type, and the length is fixed
    let arr2 = [3; 5]; // [3, 3, 3, 3, 3]
    let first = arr[0]; // Accessing array elements
    println!("The value of first is: {first}"); // Accessing array elements
    // println!("The value of arr[0] is: {arr[0]}"); // Error: array indices not allowed in format strings
}