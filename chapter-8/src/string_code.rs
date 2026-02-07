use unicode_segmentation::UnicodeSegmentation;

pub fn run_string_examples() {
    // Strings are stored as a collection of UTF-8 encoded bytes

    println!("\n========= String examples =========");

    let s1: String = String::new();
    let s2: &str = "initial contents";
    let s3: String = s2.to_string();
    let s4: String = String::from("initial contents");
    println!("s4: {}", s4);

    let mut s = String::from("foo");
    // Append string
    s.push_str("bar");
    // Append character
    s.push('!');
    println!("s: {}", s);

    // Concatenating using +
    let s5: String = String::from("Hello, ");
    let s6: String = String::from("world!");
    // Moving ownership from s5 to s7 + copying everything from s6
    let s7: String = s5 + &s6;
    println!("s7: {}", s7);
    // borrow of moved value s5 error
    // println!("s5: {}", s5);

    let s8: String = String::from("Hello, ");
    let s9: String = String::from("world!");
    // Concatenating using format! macro
    // It doesn't take ownership of s1 and s2 => we can use them later
    let s10: String = format!("{}{}", s8, s9);
    println!("s10: {}, s8: {}, s9: {}", s10, s8, s9);

    // Namaste in Hindi:
    let hello: String = String::from("नमस्ते");

    // Bytes
    for b in hello.bytes() {
        print!("{}, ", b);
    }
    // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]

    // Scalar values
    for c in hello.chars() {
        print!("{}, ", c);
    }
    // [म, स, ्, त, े,]

    println!();

    // Grapheme clusters - that's what we usually expect (a letter, a character)
    for g in hello.graphemes(true) {
        print!("{}, ", g);
    }
    // [न, म, स्, ते]
}
