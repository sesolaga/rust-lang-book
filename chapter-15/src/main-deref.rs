use std::ops::Deref;

// Tuple struct that holds one generic value.
struct MyBox<T>(T);

// Difference with Box is that x is not stored on the heap,
// but we are focusing on deref operator, so we don't care.
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    // Associated type Target = our generic T
    type Target = T;

    // Equivalent: fn deref(&self) -> &T {
    fn deref(&self) -> &Self::Target {
        // The first item in our tuple
        &self.0
    }
}

fn main() {
    // -- Deref -------------------------------------------
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // The difference is: yy is pointing to a copy of 5 because 5 is a primitive:
    // a value is copied instead of ownership being transferred.
    let yy = Box::new(x);
    assert_eq!(5, *yy);

    let z = MyBox::new(x);
    // Equivalent: assert_eq!(5, *(z.deref()));
    // Rust does this automatically => we don't have to think
    // about whether or not to call the deref method => allows
    // us to treat regular references and types that implement
    // the Deref trait - the same.
    assert_eq!(5, *z);

    // Implicit deref coercion with functions and methods
    let m = MyBox::new(String::from("Rust"));
    // This works although we're passing a ref to MyBox
    // and hello expects a string slice.
    hello(&m);
    // happens at compile time: &MyBox<String> -> &String -> &str

    // If Rust didn't have automatic deref coercion
    // hello(&(*m)[..]); // [..] - full range of a String
}

// Implicit deref coercion with functions and methods
fn hello(name: &str) {
    println!("Hello, {}!", name);
}
