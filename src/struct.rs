struct A {
    field1: i32,
    field2: String,
}

struct T(i32, String); // Tuple struct without field names

struct N; // Struct without any fields

fn main() {
    // Init struct
    let mut a = A {
        field1: 42,
        field2: String::from("Hello"),
    };
    println!("a.field1: {}, a.field2: {}", a.field1, a.field2);

    a.field1 += 1; // Update field
    println!("a.field1: {}, a.field2: {}", a.field1, a.field2);

    let b = build(String::from("World")); // Using shorthand init
    println!("b.field1: {}, b.field2: {}", b.field1, b.field2);

    let c = A {
        field1: 100,
        ..b // Use the rest of the fields from b
    };
    println!("c.field2: {}", c.field2);

    let t = T(1, String::from("Hello")); // Tuple struct
    println!("t.0: {}, t.1: {}", t.0, t.1); // Access tuple struct fields
    let T(x, y) = t; // Destructure tuple
    println!("x: {}, y: {}", x, y);

    let n = N; // Instantiate struct without fields
}

fn build(field2: String) -> A {
    // Shorthand init
    A {
        field1: 0,
        field2,
    }
}