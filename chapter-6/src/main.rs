fn main() {
    let x = Some(3);

    // We have to specify all variants of enum Option
    match x {
        Some(num) => println!("{} with match pattern", num),
        None => println!("x is none"),
    }

    // If we don't need to handle the "None" variant
    // we can use "if let" syntax

    if let Some(num) = x {
        println!("{} with if let syntax", num)
    }
}
