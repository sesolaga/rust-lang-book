pub fn run_vector_examples() {
    println!("\n========= Vector examples =========");

    // Regular array
    let a = [1, 2, 3];

    // Vector of i32
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    println!("v {:#?}", v);

    {
        // Macro for vector initialization
        let v2 = vec![1, 2, 3];

        println!("v2 {:#?}", v2);

        // v2 will be dropped when it goes out of scope
        // because vecors are allocated on the heap.
        // Also, all elements inside the vector will be dropped
    }

    let v3 = vec![1, 2, 3, 4, 5];

    let third = &v[2];
    println!("The third element is {}", third);

    // Runtime error
    // let tenth = &v[9];

    // Safe method to access the array element
    match v.get(9) {
        Some(tenth) => println!("The tenth element is {}", tenth),
        None => println!("There is no tenth element"),
    }

    // Ownership problems
    // Immutable borrow
    // let second = &v[2];

    // Mutable borrow
    // This can potentiall lead to a new memory allocation and
    // move all vector elements to a new location => second will
    // point to a stale memory address
    // v.push(4);

    // Use of immutable borrow - compile error!
    // println!("second {}", second);

    // Iteration
    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{}", i);
    }

    // Storing enums
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    match &row[1] {
        SpreadsheetCell::Int(i) => println!("{}", i),
        _ => println!("Not an integer!"),
    };
}
