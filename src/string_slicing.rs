fn main() {
    let s = String::from("Hello, world!");
    let hello = &s[..5]; // Hello
    let world = &s[7..12]; // world
    println!("{}", hello);
    println!("{}", world);
}