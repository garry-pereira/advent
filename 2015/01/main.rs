use std::fs::File;

fn main() {
    let file = File::open("input.txt");

    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Problem opening: {:?}", error),
    };
}
