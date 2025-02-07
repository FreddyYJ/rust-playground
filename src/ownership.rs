fn main() {
    let mut s1 = "hello";
    // s1.push_str(", world!"); // Error: Cannot modify a string "literal type"
    let s2 = s1;
    println!("The value of s2 is: {s2}");
    // println!("The value of s1 is: {s1}"); // Error: Value moved from s1 to s2, s1 is no longer valid

    let s1 = String::from("hello");
    s1.push_str(", world!"); // Work fine for "String" type
    let s2 = s1;
    println!("The value of s2 is: {s2}");
    // println!("The value of s1 is: {s1}"); // Error: Value moved from s1 to s2, s1 is no longer valid

    let s1 = String::from("hello");
    take_ownership(s1);
    // println!("The value of s1 is: {s1}"); // Error: take_ownership takes the ownership of s1, no longer valid

    let mut s1 = String::from("hello");
    s1 = take_and_give_back(s1);
    println!("The value of s1 is: {s1}"); // take_and_give_back returns the s1 value, s1 gets the ownership back

    let a = 5;
    let b = a;
    println!("The value of a is: {a}");
    println!("The value of b is: {b}"); // Work fine for primitive type
}

fn take_ownership(s: String) {
    println!("The value of s is: {s}");
}

fn take_and_give_back(s: String) -> String {
    println!("The value of s is: {s}");
    s // Return s, so the ownership is moved back to the caller
}

fn take_and_give_back_two(s1: String, s2: String) -> (String, String) {
    println!("The value of s1 is: {s1}");
    println!("The value of s2 is: {s2}");
    (s1, s2) // Return tuple of s1 and s2, so the ownership of two variables is moved back to the caller
}