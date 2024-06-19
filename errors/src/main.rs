use std::{
    fs::{self, File},
    io::ErrorKind,
};

fn main() {
    // let f = File::open("hello.txt").expect("File not found!");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Some problem: {}", e),
    //         },
    //         other_err => {
    //             panic!("other error: {}", other_err)
    //         }
    //     },
    // };
    println!("{:?}", read_username_from_file());
}

fn read_username_from_file() -> Result<String, std::io::Error> {
    fs::read_to_string("hello.txt")
}
