use std::collections::HashMap;

pub fn run_hashmap_examples() {
    println!("\n========= Hashmap examples =========");

    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores = HashMap::new();

    // Passing in blue and yellow with moving ownership
    scores.insert(blue, 10);
    scores.insert(yellow, 50);
    // error: borrow of moved value
    // println!("blue: {}", blue);

    // Get individual values out of a hashmap
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    // Iterating
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Updating

    // Overwriting the key
    scores.insert(String::from("Blue"), 20);

    // No overwriting
    scores.entry(String::from("Blue")).or_insert(20);

    // Update based on old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
    // {"hello": 1, "world": 2, "wonderful": 1}
}
