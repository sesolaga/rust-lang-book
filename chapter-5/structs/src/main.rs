#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// We can define multiple impl blocks
impl Rectangle {
    // Define an associated function (doesn't have a "self" argument)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let smaller = Rectangle {
        width: 29,
        height: 49,
    };

    let wont_fit = Rectangle {
        width: 29,
        height: 51,
    };

    println!("rect {:#?}", rect1);
    println!("rect area: {}", rect1.area());

    println!("smaller {:#?}", smaller);
    println!("wont fit {:#?}", wont_fit);

    println!("will smaller fit? {}", rect1.can_hold(&smaller));
    println!("will wont_fit fit? {}", rect1.can_hold(&wont_fit));

    let sq = Rectangle::square(4);
    println!("square: {:#?}", sq);
}
