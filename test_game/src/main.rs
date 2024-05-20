use qwt;
use std::fs;
mod experiment;

fn main() {
    let name = "Simon";
    println!("Hello, {name}!");
    println!("Hello, world!");

    println!("test");
    println!("Yeeehaw!");
    experiment::print_greeting();
    let file_path = "words.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Das Lesen der Datei ist gescheitert.");
    println!("Diese Wörter standen in der Datei: \n{contents}");
}
