use std::fs::File;
use std::io::{self, Read};

fn main() {
    // first we get a Result from File::open
    let file_result: Result<File, io::Error> = File::open("input.txt");

    // then we match the Result with the variants: Ok(file) and Err(error)
    let mut file: File = match file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem: {:?}", error),
    };

    // create mutable empty string
    let mut input: String = String::new();

    // read the file, match the variants, pass in our string reference to hold read
    input = match file.read_to_string(&mut input) {
        Ok(_) => input,
        Err(error) => panic!("Problem: {:?}", error),
    };

    // find the first floor santa goes in the basement
    let mut basement_check: bool = true;
    let mut basement_floor: i32 = 0;

    // at this point we have the input in a variable called input
    let mut floor_counter = 0;
    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => floor_counter = floor_counter + 1,
            ')' => floor_counter = floor_counter - 1,
            _ => continue,
        }
        if floor_counter == -1 && basement_check == true{
            basement_floor = 1 + i as i32;
            basement_check = false;
        }
    }
    println!("go to floor: {}", floor_counter);
    println!("enter basement @ instruction: {}", basement_floor);
}
