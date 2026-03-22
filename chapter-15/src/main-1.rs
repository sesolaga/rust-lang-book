fn main() {
    // Storing 5 on the heap and a pointer b on the stack.
    let b = Box::new(5);
    println!("b = {}", b);

    // Three reasons to use Box:
    // 1. When a type's exact size can't be known at compile time and we
    //    want to use the value of that type in the context that requires knowing exact size.
    // 2. When we have a large amount of data and we want to transfer ownership of the data without
    //    copying.
    // 3. When we own a value and only care that the value implements a specific trait rather than
    //    it being a specific type (this is known as trait object - chapter 17)
}
