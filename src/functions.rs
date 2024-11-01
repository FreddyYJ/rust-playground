fn main() {
    let a = 5;
    let ret = another_func(a);
    println!("The value of ret is: {ret}");

    // Block expression
    let b = {
        let a = 3;
        a + 1 // No semicolon means it is an expression. It will return the value
    };
    println!("The value of b is: {b}");
}

fn another_func(a: i32) -> i32 {
    println!("The value of a is: {a}");
    a + 1 // No semicolon means it is an expression. It will return the value
}