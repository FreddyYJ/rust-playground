fn main() {
    let mut i=0;

    // Loop with return value
    // Loop will repeat infinitely if there is no break
    let result = loop {
        println!("i is {i}");
        i+=1;
        if i==5 {
            break i;
        }
    };
    println!("result is {result}");

    // Loop with label
    let mut j=1;
    let result = 'outer: loop {
        j+=1;
        println!("j is {j}");
        'inner: loop {
            j*=2;
            println!("j is {j}");
            break 'outer; // This will break the outer loop. It automatically breaks the inner loop
        }
    };

    // While
    let mut k=0;
    while k<5 {
        println!("k is {k}");
        k+=1;
    }

    // For with list
    let arr = [10, 20, 30, 40, 50];
    for element in arr {
        println!("element is {element}");
    }

    // For with range
    for element in (1..5).rev() { // (1..5) is a range from 1 to 5, and .rev() reverses it
        println!("element is {element}");
    }
}