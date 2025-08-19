struct A {
    field1: i32,
    field2: String,
}

// Method implementations
impl A {
    fn set_field1(&mut self, value: i32) {
        self.field1 = value;
    }

    fn set_field2(&mut self, value: String) {
        self.field2 = value;
    }

    fn sign_field1(&self) -> bool {
        self.field1 > 0
    }

    // No self
    fn build(field1: i32, field2: String) -> Self {
        A { field1, field2 }
    }
}

fn main() {
    let mut a = A {
        field1: 0,
        field2: String::from("Hello"),
    };

    // Methods
    a.set_field1(10);
    a.set_field2(String::from("World"));

    println!("a.field1: {}, a.field2: {}", a.field1, a.field2);

    let is_signed = a.sign_field1();
    println!("is_signed: {}", is_signed);

    // Builder
    let b = A::build(20, String::from("Builder"));
    println!("b.field1: {}, b.field2: {}", b.field1, b.field2);
}