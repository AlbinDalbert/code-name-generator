#[macro_use] extern crate text_io;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use rand::seq::SliceRandom;


fn main() {

    let mut adjectives: Vec<String> = Vec::new();
    let mut nouns: Vec<String> = Vec::new();
    

    if let Ok(lines) = read_lines("adjectives.txt") {
        for line in lines {
            if let Ok(word) = line {
                adjectives.push(word);
            }
        }
    }
    
    if let Ok(lines) = read_lines("nouns.txt") {
        for line in lines {
            if let Ok(word) = line {
                nouns.push(word);
            }
        }
    }

    loop {

        let mut code_word: String = String::new();

        code_word.push_str(&adjectives.choose(&mut rand::thread_rng()).unwrap().to_string());
        code_word.push(' ');
        code_word.push_str(&nouns.choose(&mut rand::thread_rng()).unwrap().to_string());        

        println!("{}", code_word);
        
        let _s: String = read!("{}\n");
    }
}



fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}