use std::fmt::{Debug, Display};

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }

    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}, by {}", self.username, self.content)
    }
}

pub struct Book {
    pub content: String,
    pub author: String,
}

impl Summary for Book {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub trait Summary {
    // Method with no default impl
    fn summarize_author(&self) -> String;
    // Method with default implementation
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// Syntax sugar
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// No sugar: a trait bound
pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// If we want to require the exact same type T
pub fn notify3<T: Summary>(item1: &T, item2: &T) {
    println!(
        "Breaking news! {}, {}",
        item1.summarize(),
        item2.summarize()
    );
}

// Multiple traits syntax
pub fn notify4(item1: &(impl Summary + Display), item2: &impl Summary) {
    println!("Multiple traits syntax")
}

// Multiple traits + sqm type requirement
pub fn notify5<T: Summary + Display>(item1: &T, item2: &T) {
    println!("Multiple traits syntax + same type requirement")
}

// Not pretty - too much space between function name and its parameters
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    0
}

// Same using where clause
fn some_pretty_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    0
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("bla bla bla"),
        reply: false,
        retweet: false,
    }
}

// return impl restriction: it has to return one type
// the following won't work: if and else have incompatible types
// fn returns_summarizable_not_working(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from("Some news headline"),
//             author: String::from("Author 1"),
//             content: String::from("some content"),
//         }
//     } else {
//         Tweet {
//             username: String::from("Some user"),
//             content: String::from("Some content bla"),
//             reply: false,
//             retweet: false,
//         }
//     }
// }

fn main() {
    let tweet = Tweet {
        username: String::from("@jondoe"),
        content: String::from("Hello world!"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        author: String::from("John Doe"),
        headline: String::from("The sky is falling"),
        content: String::from("The sky is not actually falling."),
    };

    let book = Book {
        author: String::from("S.Marshak"),
        content: String::from("Some interesting book content"),
    };

    println!("Tweet summary: {}", tweet.summarize());
    println!("Article summary: {}", article.summarize());
    println!("Book default summary: {}", book.summarize());

    notify(&article);

    // summarizable
    println!("{}", returns_summarizable().summarize());
}
