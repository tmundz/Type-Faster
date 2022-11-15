/*
read.rs will create a vector of phrases
that can be randomly chosen for each game
*/

use std::fs::File;
use std::io::prelude::*;

//Parses the split file
fn parse_phrase<'a, Iter: Iterator<Item = &'a str>>(split: Iter) -> Vec<String> {
    let mut phrase: String = "".to_owned();
    let mut vec: Vec<String> = Vec::new();

    for i in split {
        if i == "" {
            vec.push(phrase);
            phrase = "".to_owned();
            continue;
        } else {
            phrase = phrase + i;
            phrase = phrase + " ";
        }
    }
    vec
}

pub fn get_phrase() -> Vec<String> {
    let mut file = File::open("phrases.txt").expect("when");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Does not exist");

    let split_content = content.split("\n");
    let phrase_vec: Vec<String> = parse_phrase(split_content);

    phrase_vec
}
