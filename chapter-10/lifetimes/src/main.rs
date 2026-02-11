use std::fmt::Display;

struct ImportantExcerpt<'a> {
    part: &'a str,
}

// All output lifetimes will be the lifetime of self
impl<'a> ImportantExcerpt<'a> {
    fn return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);

    // Example 2
    let string3 = String::from("efgh");
    {
        let string4 = String::from("ijk");

        let result = longest(string3.as_str(), string4.as_str());
        println!("Valid: the longest string is {}", result);
    }

    // Example 3 - invalid
    // let string3 = String::from("efgh");
    // let result;
    // {
    //     let string4 = String::from("ijk");
    //
    //     // Error: borrowed value (string4) does not live long enough
    //     result = longest(string3.as_str(), string4.as_str());
    // }
    // println!("Valid: the longest string is {}", result);

    // Struct example
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel
        .split('.')
        .next()
        .expect("Could not find first sentence");

    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // Static lifetime
    let s: &'static str = "I have a static lifetime";
}

// Sometimes the compiler cannot do automatic lifetime elision...
// So, we need to help it =>
//
// Generic lifetime annotation: tick a, b, c... (could be apple, banana, cucumber)
// They don't change the lifetime, they just create relationships
// between the lifetimes of multiple references.
//
// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime
//
// If x has a smaller lifetime than y, then the return value will have a lifetime of x.
// if y has a smaller - then of y.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// 1. Each parameter that is a reference gets its own lifetime parameter.
//
// 2. If there is exactly one input lifetime parameter, that lifetime is
//    assigned to all output lifetime parameters.
//
// 3. If there are multiple input lifetime parameters, but one of them is
//    &self or &mut self, the lifetime of self is assigned to all output
//    lifetime parameters.
fn first_word(s: &str) -> &str {
    // fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
