// It's worth noting that the drop fn in std is just the empty function, no magic compiler sauce here.
// It simply takes ownership of the value and make it go out of scope.

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointers created.");

    // Calling drop method directly is not allowed
    let e = CustomSmartPointer {
        data: String::from("some data"),
    };

    println!("CustomeSmartPointer e created.");
    // Explicit destructor calls are not allowed
    // e.drop();

    // Instead - use drop from std:
    drop(e);
    println!("CustomSmartPointer dropped before the end of main");
}
