use std::fs::File;
use std::io::prelude::*;

pub fn get_file() {
    let mut file = File::open("phrases.txt").expect("when");

    let mut content = String::new();
    file.read_to_string(&mut content).expect("Does not exist");
    println!("{}", content);
}
