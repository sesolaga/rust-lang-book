enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    // use of a moved value 'a' because line 10: 'b' becomes an onwer of 'a'
    // let c = Cons(4, Box::new(a));
}
