use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // Rc::clone doesn't make a deep copy, it only increments the reference counter.
    let b = Cons(3, Rc::clone(&a));
    // Equivalent: Rc::clone(&a) ~ a.clone()
    let c = Cons(4, Rc::clone(&a));
}
