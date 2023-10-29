use std::fs::File;

fn main() {
    let file = File::open("input.txt");

    // use match to unwrap Result<Ok(file), Err(error)>
    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Problem opening: {:?}", error),
    };
}
