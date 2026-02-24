#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // into_iter - returns iterator that takes ownership of the vector,
    // it will return owned types;
    // It is also an example of how closures capture their environment:
    // show_size is not defined within the closure, but we still have access to it
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    // Printing
    for value in v1_iter {
        println!("Got: {}", value);
    }

    // map (adapter)
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);

    // filter (adapter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();

        // sum (consumer)
        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_my_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                }
            ],
        );
    }
}
