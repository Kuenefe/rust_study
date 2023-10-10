use std::fs::File;
use std::io::{Error, ErrorKind};

fn main() {
    let greeting_file_result: Result<File, Error> = File::open("hello.txt");

    let greeting_file: File = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    let my_file: File = File::open("myfile.txt").unwrap_or_else(|error: Error|{
        if error.kind() == ErrorKind::NotFound {
            File::create("myfile.txt").unwrap_or_else(|error: Error| {
                panic!("Problem: {:?}", error);
            })
        } else {
            panic!("Problem: {:?}", error);
        }
    });

}
