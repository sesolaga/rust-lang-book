use std::fs::{self, File};
use std::io;
use std::io::Read;

fn read_username_from_file_verbose() -> Result<String, io::Error> {
    // Verbose
    // let f = File::open("hello.txt");

    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // Succinct
    // Success => f is a File, Fails => return Err(e)
    let mut f = File::open("hello.txt")?;

    let mut s = String::new();

    // Verbose
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }

    // Succinct
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_shorter() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn main() {
    // Verbose
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    // Succinct
    let f2 = File::open("hello.txt").unwrap();

    // Expect is the same as unwrap but it allows to specify the error message on panick
    let f3 = File::open("hello.txt").expect("Failed to open hello.txt");

    let name = read_username_from_file().unwrap();

    println!("name: {}", name);
}
