use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("1. Using fs::read_to_string");
    let f1 = read_file_with_fs_method();
    println!("Content: {f1:?}");

    println!("\n2. Using match");
    let f2 = read_file_with_match();
    println!("Content: {f2:?}");

    println!("\n3. Propagating errors with ?");
    let f3 = read_file_with_question_mark_operator()?;
    println!("Content: {f3}");

    println!("\n4. Using unwrap_or_else");
    read_file_with_unwrap_or_else();

    println!("\n5. Using expect");
    read_file_with_expect();

    println!("\n6. Using unwrap");
    read_file_with_unwrap();
    
    Ok(())
}

fn read_file_with_fs_method() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn read_file_with_question_mark_operator() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}


fn read_file_with_match() -> Result<String, io::Error> {
    let file_result = File::open("hello.txt");
    let mut file = match file_result {
        Ok(f) => f,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };
    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => Ok(content),
        Err(e) => Err(e)
    }
}

fn read_file_with_unwrap_or_else() {
    let file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
}

fn read_file_with_unwrap() {
    let file = File::open("hello.txt").unwrap();
}

fn read_file_with_expect() {
    let file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}


