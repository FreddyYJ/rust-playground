fn main() {
    let a=0;
    let b=1;
    if a == b {
        println!("a is equal to b");
    } else if a < b {
        println!("a is lesser to b");
    } else {
        println!("a is greater to b");
    }

    // Condition should be a boolean
    // if a {
    //     println!("a is true");
    // } else {
    //     println!("a is false");
    // }

    // Trenary operators
    let cond=true;
    let x=if cond {5} else {6};
    println!("x is {x}");
}