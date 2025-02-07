fn main() {
    let mut s1 = String::from("hello");
    let s2 = &s1; // reference of s1
    println!("The value of s2 is: {s2}");
    println!("The value of s1 is: {s1}"); // s1 is still valid because s2 is a reference to s1

    // s2.push_str(", world!"); // Error: Cannot modify because s2 is an "immutable" reference
    let s2 = &mut s1; // "mutable" reference of s1
    s2.push_str(", world!"); // Now work fine for "mutable" reference

    // let mut s3 = &mut s1; // Error: Cannot have multiple "mutable" references

    ref_print(&mut s1);
}

fn ref_print(s: &mut String) {
    s.push_str(", world!");
    println!("The value of s is: {s}");
}