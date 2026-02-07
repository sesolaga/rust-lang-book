#[derive(Debug)]
struct PointWithDiffTypes<T: Copy, U: Copy> {
    x: T,
    y: U,
}

impl<T: Copy, U: Copy> PointWithDiffTypes<T, U> {
    fn mixup<V: Copy, W: Copy>(
        &self,
        other: &PointWithDiffTypes<V, W>,
    ) -> PointWithDiffTypes<T, W> {
        PointWithDiffTypes {
            x: self.x,
            y: other.y,
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

struct Point<T> {
    x: T,
    y: T,
}

impl<U> Point<U> {
    // This method is going to be available to points with any type
    fn x(&self) -> &U {
        &self.x
    }
}

impl Point<f64> {
    // This method is only going to be available to points with f64 type
    fn y(&self) -> f64 {
        self.y
    }
}

///////////////////////////////////////////////////////////////////////////////

fn get_largest<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    largest
}

///////////////////////////////////////////////////////////////////////////////

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let largest_number = get_largest(number_list);
    println!("The largest number is {}", largest_number);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let largest_char = get_largest(char_list);
    println!("The largest character is {}", largest_char);

    // Generics with structs
    let p1 = PointWithDiffTypes { x: 5, y: 10 };
    let p2 = PointWithDiffTypes { x: 5.0, y: 10.0 };
    let p3 = PointWithDiffTypes { x: 5, y: 10.0 };

    // Generics inside enums
    enum Option<T> {
        Some(T),
        None,
    }

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    // Point
    let p4 = Point { x: 5, y: 10 };
    // p4.x() is available, p4.y() - is not

    let p5 = Point { x: 5.0, y: 10.0 };
    // p5.x() is available, p5.y() - is available

    // Mixup
    let p6 = PointWithDiffTypes { x: 5, y: 10.4 };
    let p7 = PointWithDiffTypes { x: "Hello", y: 'c' };

    let p8 = p6.mixup(&p7);
    println!("Mixup point: {:?}", p8);
    println!("p6 point: {:?}", p6);
    println!("p7 point: {:?}", p7);
}
