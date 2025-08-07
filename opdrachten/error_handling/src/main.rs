use std::fs::{self, File};
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    // panic!("Crash and burn!")

    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    // let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {e:?}"),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {other_error:?}")
    //         }
    //     }
    // };

    // let f = File::open("hello.txt").unwrap();
}

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
