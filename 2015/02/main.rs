use std::fs::File;
use std::io::Read;

fn main() {
    // try to open file
    let mut file: File = match File::open("input.txt") {
        Ok(file) => file,
        Err(error) => panic!("Problem: {}", error),
    };

    // read it to string
    let mut input: String = String::new();
    input = match file.read_to_string(&mut input) {
        Ok(_) => input,
        Err(error) => panic!("Problem: {}", error),
    };

    //delete
    println!("{}", input);
}
