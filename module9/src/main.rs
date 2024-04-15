use std::fs::File;
// use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    // panic!("crash and burn");
    // println!("Hello, world!");
    
    // let greeting_file_result = File::open("Hello.txt")
            // .expect("hello.txt should be include in this project.");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error);
    //         }
    //     },
    // };
    
    let res = read_username_from_file();
    match res {
        Ok(s) => println!("{}", s),
        Err(e) => println!("{}", e),
    };
    match read_username_from_file2() {
        Ok(s) => println!("{}", s),
        Err(e) => println!("{}", e),
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    return match username_file.read_to_string(&mut username)
    {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    };
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file3() -> Result<String,io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}